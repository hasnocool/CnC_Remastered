// Filename: crates/renderer/src/bin/asset_validate.rs
//! Validate user-supplied retail TD/RA scenarios, MIX archives, palettes and terrain atlases.
//!
//! This tool never writes into or copies proprietary game data into the repository.
//!
//! Examples:
//!   cargo run -p renderer --bin asset-validate -- --variant td --scenario /games/td/SCG01EA.INI --map-bin /games/td/SCG01EA.BIN --palette /games/td/TEMPERAT.PAL --mix /games/td/TEMPERAT.MIX
//!   cargo run -p renderer --bin asset-validate -- --variant ra --scenario /games/ra/SCG01EA.INI --palette /games/ra/TEMPERAT.PAL --mix /games/ra/TEMPERAT.MIX --mix /games/ra/LOCAL.MIX

use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs,
    path::PathBuf,
};

use renderer::TemplateAssetProvider;
use rts_core::{GameVariant, template_catalog::TemplateTheater};
use rts_formats::{
    mix::{MixArchive, filename_crc},
    palette_rgba::{Palette256, PaletteEncoding},
    ra_map_pack::RaTemplateMap,
    td_map::TdTemplateMap,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MAX_VALIDATION_CELLS: usize = 1_048_576;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant { Td, Ra }

#[derive(Debug)]
struct Options {
    variant: Variant,
    scenario: PathBuf,
    td_map_bin: Option<PathBuf>,
    palette: PathBuf,
    palette_encoding: PaletteEncoding,
    mix_files: Vec<PathBuf>,
    theater_override: Option<TemplateTheater>,
}

#[derive(Debug, Default)]
struct IniDocument {
    sections: BTreeMap<String, Vec<(String, String)>>,
}

impl IniDocument {
    fn parse(source: &str) -> Self {
        let mut document = Self::default();
        let mut section = String::new();

        for raw_line in source.lines() {
            let line = raw_line.trim().trim_start_matches('\u{feff}').trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') && line.len() >= 2 {
                section = line[1..line.len() - 1].trim().to_ascii_lowercase();
                document.sections.entry(section.clone()).or_default();
                continue;
            }
            let Some((key, value)) = line.split_once('=') else { continue; };
            let key = key.trim().to_ascii_lowercase();
            let value = value.split(';').next().unwrap_or("").trim().trim_matches('"').to_owned();
            if !key.is_empty() {
                document.sections.entry(section.clone()).or_default().push((key, value));
            }
        }
        document
    }

    fn value(&self, section: &str, key: &str) -> Option<&str> {
        self.sections.get(&section.to_ascii_lowercase())?
            .iter().rev()
            .find(|(candidate, _)| candidate == &key.to_ascii_lowercase())
            .map(|(_, value)| value.as_str())
    }

    fn value_any_section(&self, key: &str) -> Option<&str> {
        let key = key.to_ascii_lowercase();
        self.sections.values().flat_map(|entries| entries.iter().rev())
            .find(|(candidate, _)| candidate == &key)
            .map(|(_, value)| value.as_str())
    }

    fn required_u32(&self, section: &str, key: &str) -> Result<u32> {
        let raw = self.value(section, key)
            .or_else(|| self.value_any_section(key))
            .ok_or_else(|| format!("scenario INI is missing [{section}] {key}"))?;
        let value = raw.parse::<u32>()
            .map_err(|_| format!("scenario INI has invalid {key}={raw:?}"))?;
        if value == 0 { return Err(format!("scenario {key} must be nonzero").into()); }
        Ok(value)
    }

    fn theater(&self) -> Result<TemplateTheater> {
        let value = self.value_any_section("theater")
            .ok_or("scenario INI is missing its Theater setting (or pass --theater)")?;
        parse_theater(value)
    }

    fn ra_mappack_base64(&self) -> Result<String> {
        let entries = self.sections.get("mappack")
            .ok_or("Red Alert scenario has no [MapPack] section")?;
        let mut chunks: Vec<(u64, &str)> = entries.iter().map(|(key, value)| {
            let order = key.parse::<u64>()
                .map_err(|_| format!("unsupported [MapPack] key {key:?}; expected numeric chunk keys"))?;
            Ok((order, value.as_str()))
        }).collect::<std::result::Result<_, String>>()?;
        chunks.sort_by_key(|(order, _)| *order);
        if chunks.is_empty() { return Err("Red Alert [MapPack] section is empty".into()); }
        if chunks.windows(2).any(|pair| pair[0].0 == pair[1].0) {
            return Err("Red Alert [MapPack] section contains duplicate chunk numbers".into());
        }
        let capacity = chunks.iter().map(|(_, value)| value.len()).sum();
        let mut encoded = String::with_capacity(capacity);
        for (_, value) in chunks { encoded.push_str(value); }
        Ok(encoded)
    }
}

fn parse_theater(value: &str) -> Result<TemplateTheater> {
    match value.trim().to_ascii_lowercase().as_str() {
        "desert" | "des" => Ok(TemplateTheater::Desert),
        "jungle" | "jun" => Ok(TemplateTheater::Jungle),
        "temperate" | "temperat" | "tem" => Ok(TemplateTheater::Temperate),
        "winter" | "win" => Ok(TemplateTheater::Winter),
        "snow" | "sno" => Ok(TemplateTheater::Snow),
        "interior" | "int" => Ok(TemplateTheater::Interior),
        other => Err(format!("unknown theater {other:?}; use desert, jungle, temperate, winter, snow, or interior").into()),
    }
}

fn parse_options() -> Result<Option<Options>> {
    let mut args = env::args().skip(1);
    let mut variant = None;
    let mut scenario = None;
    let mut td_map_bin = None;
    let mut palette = None;
    let mut palette_encoding = PaletteEncoding::Rgb8;
    let mut mix_files = Vec::new();
    let mut theater_override = None;

    while let Some(arg) = args.next() {
        let value = |args: &mut std::iter::Skip<std::env::Args>, name: &str| -> Result<String> {
            args.next().ok_or_else(|| format!("{name} requires a value").into())
        };
        match arg.as_str() {
            "--help" | "-h" => { print_help(); return Ok(None); }
            "--variant" => variant = Some(match value(&mut args, "--variant")?.to_ascii_lowercase().as_str() {
                "td" | "tiberian-dawn" => Variant::Td,
                "ra" | "red-alert" => Variant::Ra,
                other => return Err(format!("unknown variant {other:?}; expected td or ra").into()),
            }),
            "--scenario" => scenario = Some(PathBuf::from(value(&mut args, "--scenario")?)),
            "--map-bin" => td_map_bin = Some(PathBuf::from(value(&mut args, "--map-bin")?)),
            "--palette" => palette = Some(PathBuf::from(value(&mut args, "--palette")?)),
            "--palette-encoding" => palette_encoding = match value(&mut args, "--palette-encoding")?.to_ascii_lowercase().as_str() {
                "rgb8" => PaletteEncoding::Rgb8,
                "rgb6" => PaletteEncoding::Rgb6,
                other => return Err(format!("unknown palette encoding {other:?}; expected rgb8 or rgb6").into()),
            },
            "--mix" => mix_files.push(PathBuf::from(value(&mut args, "--mix")?)),
            "--theater" => theater_override = Some(parse_theater(&value(&mut args, "--theater")?)?),
            other => return Err(format!("unknown argument {other:?}; use --help").into()),
        }
    }

    let Some(variant) = variant else { print_help(); return Err("--variant is required".into()); };
    let options = Options {
        variant,
        scenario: scenario.ok_or("--scenario is required")?,
        td_map_bin,
        palette: palette.ok_or("--palette is required")?,
        palette_encoding,
        mix_files,
        theater_override,
    };
    if options.mix_files.is_empty() { return Err("at least one --mix archive is required".into()); }
    if options.variant == Variant::Td && options.td_map_bin.is_none() {
        return Err("--map-bin is required for Tiberian Dawn editor BIN scenarios".into());
    }
    if options.variant == Variant::Ra && options.td_map_bin.is_some() {
        return Err("--map-bin is only valid for Tiberian Dawn; Red Alert terrain comes from [MapPack]".into());
    }
    Ok(Some(options))
}

fn print_help() {
    println!("CnC_Remastered retail asset validation");
    println!("  --variant td|ra                 Game variant (required)");
    println!("  --scenario FILE                 Scenario INI (required)");
    println!("  --map-bin FILE                  TD editor terrain BIN (required for td)");
    println!("  --palette FILE                  768-byte RGB palette (required)");
    println!("  --palette-encoding rgb8|rgb6    Palette component encoding (default rgb8)");
    println!("  --mix FILE                      MIX archive; repeat to define lookup order");
    println!("  --theater NAME                  Override scenario Theater metadata");
    println!("  --help                          Show this help");
    println!();
    println!("No game files are copied, modified, or bundled. Run against files you are authorized to use.");
}

fn main() {
    if let Err(error) = run() {
        eprintln!("asset-validate: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let Some(options) = parse_options()? else { return Ok(()); };
    let scenario_text = fs::read_to_string(&options.scenario)
        .map_err(|error| format!("could not read scenario {}: {error}", options.scenario.display()))?;
    let ini = IniDocument::parse(&scenario_text);
    let width = ini.required_u32("map", "width")?;
    let height = ini.required_u32("map", "height")?;
    let cell_count = checked_cell_count(width, height)?;
    let theater = match options.theater_override {
        Some(theater) => theater,
        None => ini.theater()?,
    };
    validate_variant_theater(options.variant, theater)?;

    let palette_bytes = fs::read(&options.palette)
        .map_err(|error| format!("could not read palette {}: {error}", options.palette.display()))?;
    let palette = Palette256::parse(&palette_bytes, options.palette_encoding)
        .map_err(|error| format!("palette validation failed: {error}"))?;

    let mut mix_bytes = Vec::with_capacity(options.mix_files.len());
    for path in &options.mix_files {
        mix_bytes.push(fs::read(path).map_err(|error| format!("could not read MIX {}: {error}", path.display()))?);
    }
    let archives: Vec<MixArchive<'_>> = mix_bytes.iter().enumerate().map(|(index, bytes)| {
        MixArchive::parse(bytes).map_err(|error| format!("MIX {} ({}) is invalid: {error}", index + 1, options.mix_files[index].display()))
    }).collect::<std::result::Result<_, String>>()?;
    let archive_refs: Vec<&MixArchive<'_>> = archives.iter().collect();

    let (terrain_map, atlas, template_types, cell_count) = match options.variant {
        Variant::Td => {
            let path = options.td_map_bin.as_ref().expect("validated TD map path");
            let bytes = fs::read(path).map_err(|error| format!("could not read TD BIN {}: {error}", path.display()))?;
            let map = TdTemplateMap::decode_binary(&bytes, width, height)
                .map_err(|error| format!("TD BIN validation failed: {error}"))?;
            let types: BTreeSet<u16> = map.cells().iter().map(|cell| u16::from(cell.template_type)).collect();
            let provider = TemplateAssetProvider::new(GameVariant::TiberianDawn, theater, &archive_refs);
            let (terrain, atlas) = provider.build_td_terrain_atlas(1, &map, &palette)
                .map_err(|error| format!("TD template/atlas validation failed: {error}"))?;
            let count = map.cells().len();
            (terrain, atlas, types, count)
        }
        Variant::Ra => {
            let encoded = ini.ra_mappack_base64()?;
            let map = RaTemplateMap::decode_ini_section(&encoded, width, height)
                .map_err(|error| format!("RA MapPack validation failed: {error}"))?;
            let types: BTreeSet<u16> = map.cells().iter().map(|cell| cell.template_type).collect();
            let provider = TemplateAssetProvider::new(GameVariant::RedAlert, theater, &archive_refs);
            let (terrain, atlas) = provider.build_ra_terrain_atlas(1, &map, &palette)
                .map_err(|error| format!("RA template/atlas validation failed: {error}"))?;
            let count = map.cells().len();
            (terrain, atlas, types, count)
        }
    };

    let mut mix_entries = 0usize;
    for archive in &archives { mix_entries += archive.entries().len(); }
    let mut crc_lines = Vec::with_capacity(template_types.len());
    let provider = TemplateAssetProvider::new(
        match options.variant { Variant::Td => GameVariant::TiberianDawn, Variant::Ra => GameVariant::RedAlert },
        theater,
        &archive_refs,
    );
    for template_type in &template_types {
        let filename = provider.filename_for_template(*template_type)
            .map_err(|error| format!("template catalog validation failed: {error}"))?;
        let crc = filename_crc(&filename);
        crc_lines.push((filename, crc));
    }

    let (atlas_width, atlas_height) = atlas.dimensions();
    let atlas_bytes = &atlas.image().pixels;
    let checksum = fnv1a64(atlas_bytes);
    let resolved_cells = (0..height).flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|(x, y)| terrain_map.tile_at(*x, *y).is_some())
        .count();

    println!("Retail asset validation: PASS");
    println!("Variant: {:?}", options.variant);
    println!("Scenario: {}", options.scenario.display());
    println!("Theater: {:?}", theater);
    println!("Map: {width}x{height} ({cell_count} cells)");
    println!("MIX archives: {} ({mix_entries} indexed entries)", archives.len());
    println!("Referenced template types: {}", template_types.len());
    for (filename, crc) in crc_lines {
        println!("  {filename:<16} CRC={crc:#010x} [resolved]");
    }
    println!("Palette: {} ({:?}, {} bytes)", options.palette.display(), options.palette_encoding, palette_bytes.len());
    println!("Resolved terrain cells: {resolved_cells}/{cell_count}");
    println!("Atlas: {atlas_width}x{atlas_height} RGBA8 ({} bytes)", atlas_bytes.len());
    println!("Atlas FNV-1a 64: {checksum:#018x}");
    println!("GPU upload/rendering is intentionally not attempted; this is a deterministic CPU ingestion check.");
    Ok(())
}

fn checked_cell_count(width: u32, height: u32) -> Result<usize> {
    let cells = usize::try_from(width).ok()
        .and_then(|width| usize::try_from(height).ok().and_then(|height| width.checked_mul(height)))
        .ok_or("scenario map dimensions overflow addressable memory")?;
    if cells > MAX_VALIDATION_CELLS {
        return Err(format!("scenario map has {cells} cells; validation limit is {MAX_VALIDATION_CELLS}").into());
    }
    Ok(cells)
}

fn validate_variant_theater(variant: Variant, theater: TemplateTheater) -> Result<()> {
    let valid = match variant {
        Variant::Td => matches!(theater, TemplateTheater::Desert | TemplateTheater::Jungle | TemplateTheater::Temperate | TemplateTheater::Winter),
        Variant::Ra => matches!(theater, TemplateTheater::Temperate | TemplateTheater::Snow | TemplateTheater::Interior),
    };
    if !valid { return Err(format!("theater {theater:?} is not valid for {variant:?}").into()); }
    Ok(())
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ini_sections_case_insensitively_and_strips_comments() {
        let ini = IniDocument::parse("[Map]\nWidth=64 ; width comment\nHEIGHT=32\nTheater=TEMPERATE\n");
        assert_eq!(ini.required_u32("MAP", "width").unwrap(), 64);
        assert_eq!(ini.required_u32("map", "height").unwrap(), 32);
        assert_eq!(ini.theater().unwrap(), TemplateTheater::Temperate);
    }

    #[test]
    fn concatenates_red_alert_mappack_chunks_in_numeric_order() {
        let ini = IniDocument::parse("[MapPack]\n10=K\n2=B\n1=A\n");
        assert_eq!(ini.ra_mappack_base64().unwrap(), "ABK");
    }

    #[test]
    fn rejects_non_numeric_and_empty_red_alert_mappack_sections() {
        let non_numeric = IniDocument::parse("[MapPack]\nMapPack0=AAAA\n");
        assert!(non_numeric.ra_mappack_base64().is_err());
        let empty = IniDocument::parse("[Other]\n1=AAAA\n");
        assert!(empty.ra_mappack_base64().is_err());
    }

    #[test]
    fn maps_theater_aliases_and_rejects_unknown_values() {
        assert_eq!(parse_theater("TEM").unwrap(), TemplateTheater::Temperate);
        assert_eq!(parse_theater("SNO").unwrap(), TemplateTheater::Snow);
        assert!(parse_theater("moon").is_err());
    }

    #[test]
    fn fnv_checksum_is_stable() {
        assert_eq!(fnv1a64(b""), 0xcbf29ce484222325);
        assert_eq!(fnv1a64(b"a"), 0xaf63dc4c8601ec8c);
    }
}
