//! Private profile/config access and Red Alert binary-buffer helpers from
//! REDALERT/PROFILE.CPP and TIBERIANDAWN/PROFILE.CPP. Text lookup is
//! case-insensitive and handles CRLF files; the binary cursors preserve the
//! source's asymmetric read/write end-bound checks.

use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileBuffer { bytes: Vec<u8> }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileLookup {
    /// Offset corresponding to the legacy returned char pointer.
    pub offset: Option<usize>,
    /// NUL-terminated value, or a double-NUL key list for entry=None.
    pub output: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewConfigType {
    pub digit_card: u32,
    pub irq: i32,
    pub dma: i32,
    pub port: u32,
    pub bits_per_sample: i32,
    pub channels: i32,
    pub reverse: i32,
    pub speed: i32,
    pub language: [u8; 3],
}

impl Default for NewConfigType {
    fn default() -> Self {
        Self {
            digit_card: 0, irq: 0, dma: 0, port: 0, bits_per_sample: 0,
            channels: 0, reverse: 0, speed: 0, language: [0; 3],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileError {
    InvalidUtf8,
    InvalidBufferLength,
    ReadWouldReachEnd { position: usize, length: usize, capacity: usize },
    ReadStringOutOfBounds { position: usize, encoded_length: usize, capacity: usize },
    WriteOutOfBounds { position: usize, length: usize, capacity: usize },
    StringLengthOutOfRange(usize),
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for ProfileError {}

impl ProfileBuffer {
    pub fn new(text: impl AsRef<[u8]>) -> Self {
        let mut bytes = text.as_ref().to_vec();
        if bytes.last().copied() != Some(0) { bytes.push(0); }
        Self { bytes }
    }
    pub fn empty() -> Self { Self { bytes: vec![0] } }
    pub fn as_c_bytes(&self) -> &[u8] { &self.bytes }
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.bytes.iter().position(|b| *b == 0).unwrap_or(self.bytes.len())]
    }
    pub fn as_text(&self) -> Result<&str, ProfileError> {
        std::str::from_utf8(self.as_bytes()).map_err(|_| ProfileError::InvalidUtf8)
    }

    pub fn get_string(
        &self,
        section: &str,
        entry: Option<&str>,
        default: Option<&str>,
        output_capacity: Option<usize>,
    ) -> ProfileLookup {
        let mut output = output_capacity.map(|capacity| {
            let mut buffer = vec![0u8; capacity];
            if capacity > 0 {
                if let Some(default) = default {
                    let n = default.len().min(capacity - 1);
                    buffer[..n].copy_from_slice(&default.as_bytes()[..n]);
                }
            }
            buffer
        }).unwrap_or_default();

        let Some((_, body_start, body_end)) = self.section_bounds(section) else {
            return ProfileLookup { offset: None, output };
        };

        if let Some(entry_name) = entry {
            let Some((line_start, _, line_end)) =
                self.find_entry(body_start, body_end, entry_name)
            else { return ProfileLookup { offset: None, output } };

            if output_capacity.unwrap_or(0) > 0 {
                if let Some(value) = self.line_value(line_start, line_end) {
                    let value = trim_space_tab(value);
                    let n = value.len().min(output.len().saturating_sub(1));
                    output[..n].copy_from_slice(&value[..n]);
                    output[n] = 0;
                }
            }
            ProfileLookup { offset: Some(line_start), output }
        } else {
            if !output.is_empty() {
                let mut cursor = 0usize;
                for (start, _, end) in self.line_ranges(body_start, body_end) {
                    let line = &self.as_bytes()[start..end];
                    if let Some(equal) = line.iter().position(|b| *b == b'=') {
                        let key = trim_space_tab(&line[..equal]);
                        if key.is_empty() { continue; }
                        // Source requires key length + three trailing NUL bytes.
                        if cursor + key.len() + 3 >= output.len() { break; }
                        output[cursor..cursor + key.len()].copy_from_slice(key);
                        cursor += key.len() + 1;
                    }
                }
                if cursor < output.len() { output[cursor] = 0; }
                if cursor + 1 < output.len() { output[cursor + 1] = 0; }
            }
            ProfileLookup { offset: Some(body_start), output }
        }
    }

    pub fn get_hex(&self, section: &str, entry: &str) -> u32 {
        let result = self.get_string(section, Some(entry), Some("0"), Some(16));
        let end = result.output.iter().position(|b| *b == 0).unwrap_or(result.output.len());
        let text = String::from_utf8_lossy(&result.output[..end]);
        parse_u32_radix(text.trim(), 16).unwrap_or(0)
    }

    pub fn get_int(&self, section: &str, entry: &str, default: i32) -> i32 {
        let default = default.to_string();
        let result = self.get_string(section, Some(entry), Some(&default), Some(15));
        let end = result.output.iter().position(|b| *b == 0).unwrap_or(result.output.len());
        parse_atoi(&String::from_utf8_lossy(&result.output[..end]))
    }

    pub fn write_int(&mut self, section: &str, entry: Option<&str>, value: i32) -> bool {
        self.write_string(section, entry, Some(&value.to_string()))
    }

    pub fn write_string(&mut self, section: &str, entry: Option<&str>, value: Option<&str>) -> bool {
        if section.is_empty() { return true; }

        if self.section_bounds(section).is_none() {
            if entry.is_none() || value.is_none() { return; }
            let end = self.data_len();
            let mut suffix = Vec::new();
            if end != 0 && self.bytes[end - 1] != b'\n' {
                suffix.extend_from_slice(b"\r\n");
            }
            suffix.extend_from_slice(format!("[{section}]\r\n").as_bytes());
            self.insert_at(end, &suffix);
        }

        let Some((header_start, body_start, body_end)) = self.section_bounds(section) else { return true };
        let section_end = body_end;

        if entry.is_none() {
            self.remove_range(header_start, section_end);
            return true;
        }
        let entry = entry.unwrap();

        let existing = self.find_entry(body_start, body_end, entry);
        let insertion = if let Some((line_start, _, line_end)) = existing {
            self.remove_range(line_start, line_end);
            line_start
        } else {
            self.first_entry_or_end(body_start, body_end)
        };

        if let Some(value) = value {
            let line = format!("{entry}={value}\r\n");
            self.insert_at(insertion, line.as_bytes());
        }
    }

    fn data_len(&self) -> usize {
        self.bytes.iter().position(|b| *b == 0).unwrap_or(self.bytes.len())
    }

    fn section_bounds(&self, section: &str) -> Option<(usize, usize, usize)> {
        let bytes = self.as_bytes();
        let mut cursor = 0usize;
        while cursor < bytes.len() {
            let (line_start, content_end, next) = line_bounds(bytes, cursor);
            let line = trim_line_ending(&bytes[line_start..content_end]);
            if line.first() == Some(&b'[') && line.last() == Some(&b']')
                && ascii_eq_ignore_case(&line[1..line.len() - 1], section.as_bytes())
            {
                let body_start = next;
                let mut end_cursor = body_start;
                while end_cursor < bytes.len() {
                    let (_, end, next_line) = line_bounds(bytes, end_cursor);
                    let candidate = trim_line_ending(&bytes[end_cursor..end]);
                    if candidate.first() == Some(&b'[') && candidate.last() == Some(&b']') {
                        return Some((line_start, body_start, end_cursor));
                    }
                    end_cursor = next_line;
                }
                return Some((line_start, body_start, bytes.len()));
            }
            cursor = next;
        }
        None
    }

    fn find_entry(
        &self, start: usize, end: usize, entry: &str,
    ) -> Option<(usize, usize, usize)> {
        let bytes = self.as_bytes();
        let mut cursor = start;
        while cursor < end {
            let (line_start, content_end, next) = line_bounds(bytes, cursor);
            let line = trim_line_ending(&bytes[line_start..content_end]);
            if let Some(equal) = line.iter().position(|b| *b == b'=') {
                let key = trim_space_tab(&line[..equal]);
                if ascii_eq_ignore_case(key, entry.as_bytes()) {
                    return Some((line_start, content_end, next.min(end)));
                }
            }
            cursor = next;
        }
        None
    }

    fn line_ranges(&self, start: usize, end: usize) -> Vec<(usize, usize, usize)> {
        let bytes = self.as_bytes();
        let mut ranges = Vec::new();
        let mut cursor = start;
        while cursor < end {
            let bounds = line_bounds(bytes, cursor);
            ranges.push((bounds.0, bounds.1, bounds.2.min(end)));
            cursor = bounds.2;
        }
        ranges
    }

    fn line_value(&self, start: usize, end: usize) -> Option<&[u8]> {
        let line = &self.as_bytes()[start..end];
        let equal = line.iter().position(|b| *b == b'=')?;
        let mut value = &line[equal + 1..];
        if let Some(carriage) = value.iter().position(|b| *b == b'\r') {
            value = &value[..carriage];
        } else if let Some(newline) = value.iter().position(|b| *b == b'\n') {
            value = &value[..newline];
        }
        Some(trim_space_tab(value))
    }

    fn first_entry_or_end(&self, start: usize, end: usize) -> usize {
        for (line_start, _, _) in self.line_ranges(start, end) {
            if self.as_bytes()[line_start..].first().is_some() {
                return line_start;
            }
        }
        end
    }

    fn remove_range(&mut self, start: usize, end: usize) {
        let end = end.min(self.data_len());
        if start >= end { return; }
        self.bytes.drain(start..end);
        self.ensure_nul();
    }

    fn insert_at(&mut self, offset: usize, bytes: &[u8]) {
        let offset = offset.min(self.data_len());
        self.bytes.splice(offset..offset, bytes.iter().copied());
        self.ensure_nul();
    }

    fn ensure_nul(&mut self) {
        if self.bytes.last().copied() != Some(0) { self.bytes.push(0); }
    }
}

fn line_bounds(bytes: &[u8], start: usize) -> (usize, usize, usize) {
    let end = bytes[start..].iter().position(|b| *b == b'\n')
        .map(|p| start + p).unwrap_or(bytes.len());
    let next = if end < bytes.len() { end + 1 } else { end };
    (start, end, next)
}
fn trim_line_ending(bytes: &[u8]) -> &[u8] {
    if bytes.last().copied() == Some(b'\r') { &bytes[..bytes.len() - 1] } else { bytes }
}
fn trim_space_tab(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|b| *b != b' ' && *b != b'\t').unwrap_or(bytes.len());
    let end = bytes.iter().rposition(|b| *b != b' ' && *b != b'\t')
        .map(|i| i + 1).unwrap_or(start);
    &bytes[start..end]
}
fn ascii_eq_ignore_case(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.eq_ignore_ascii_case(y))
}
fn parse_u32_radix(text: &str, radix: u32) -> Option<u32> {
    let text = text.trim_start();
    let text = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")).unwrap_or(text);
    u32::from_str_radix(text.trim(), radix).ok()
}
fn parse_atoi(text: &str) -> i32 {
    let text = text.trim_start();
    let (negative, digits) = if let Some(v) = text.strip_prefix('-') {
        (true, v)
    } else if let Some(v) = text.strip_prefix('+') {
        (false, v)
    } else { (false, text) };
    let digits: String = digits.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { return 0; }
    let value = digits.parse::<i64>().unwrap_or(i64::MAX);
    (if negative { -value } else { value }).clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

pub fn read_private_config(profile: &ProfileBuffer) -> (NewConfigType, bool) {
    let mut config = NewConfigType::default();
    config.digit_card = profile.get_hex("Sound", "Card");
    config.irq = profile.get_int("Sound", "IRQ", 0);
    config.dma = profile.get_int("Sound", "DMA", 0);
    config.port = profile.get_hex("Sound", "Port");
    config.bits_per_sample = profile.get_int("Sound", "BitsPerSample", 0);
    config.channels = profile.get_int("Sound", "Channels", 0);
    config.reverse = profile.get_int("Sound", "Reverse", 0);
    config.speed = profile.get_int("Sound", "Speed", 0);
    let language = profile.get_string("Language", Some("Language"), None, Some(3));
    let length = language.output.iter().position(|b| *b == 0)
        .unwrap_or(language.output.len()).min(2);
    config.language[..length].copy_from_slice(&language.output[..length]);
    let no_sound_card = config.digit_card == 0 && config.irq == 0 && config.dma == 0;
    (config, no_sound_card)
}

/// Binary serialization helpers present in the Red Alert PROFILE.CPP only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryProfileBuffer {
    bytes: Vec<u8>,
    position: usize,
    high_water: usize,
    writable: bool,
}

impl BinaryProfileBuffer {
    pub fn reader(bytes: &[u8]) -> Self {
        Self { bytes: bytes.to_vec(), position: 0, high_water: 0, writable: false }
    }
    pub fn writer(capacity: usize) -> Self {
        Self { bytes: vec![0; capacity], position: 0, high_water: 0, writable: true }
    }
    pub fn position(&self) -> usize { self.position }
    pub fn length(&self) -> usize { self.high_water }
    pub fn set_position(&mut self, position: usize) -> usize { self.position = position; position }
    pub fn bytes(&self) -> &[u8] { &self.bytes[..self.high_water.min(self.bytes.len())] }

    /// Read_Bin_Num rejects reads whose new position equals the buffer length.
    pub fn read_num(&mut self, length: usize) -> Result<Vec<u8>, ProfileError> {
        if self.writable || length == 0 || length > 4
            || self.position.checked_add(length).is_none_or(|end| end >= self.bytes.len())
        {
            return Err(ProfileError::ReadWouldReachEnd {
                position: self.position, length, capacity: self.bytes.len(),
            });
        }
        let end = self.position + length;
        let result = self.bytes[self.position..end].to_vec();
        self.position = end;
        self.high_water = self.high_water.max(end);
        Ok(result)
    }

    pub fn write_num(&mut self, source: &[u8]) -> Result<(), ProfileError> {
        if !self.writable || source.is_empty() || source.len() > 4 {
            return Err(ProfileError::InvalidBufferLength);
        }
        let end = self.position.checked_add(source.len())
            .ok_or(ProfileError::WriteOutOfBounds {
                position: self.position, length: source.len(), capacity: self.bytes.len(),
            })?;
        if end > self.bytes.len() {
            return Err(ProfileError::WriteOutOfBounds {
                position: self.position, length: source.len(), capacity: self.bytes.len(),
            });
        }
        self.bytes[self.position..end].copy_from_slice(source);
        self.position = end;
        self.high_water = self.high_water.max(end);
        Ok(())
    }

    /// Stored form is one-byte length, content bytes, one terminating NUL.
    pub fn write_string(&mut self, text: &[u8]) -> Result<(), ProfileError> {
        if !self.writable { return Err(ProfileError::InvalidBufferLength); }
        if text.len() > 255 { return Err(ProfileError::StringLengthOutOfRange(text.len())); }
        let needed = text.len() + 2;
        let end = self.position.checked_add(needed)
            .ok_or(ProfileError::WriteOutOfBounds {
                position: self.position, length: needed, capacity: self.bytes.len(),
            })?;
        if end > self.bytes.len() {
            return Err(ProfileError::WriteOutOfBounds {
                position: self.position, length: needed, capacity: self.bytes.len(),
            });
        }
        self.bytes[self.position] = text.len() as u8;
        self.bytes[self.position + 1..self.position + 1 + text.len()].copy_from_slice(text);
        self.bytes[self.position + 1 + text.len()] = 0;
        self.position = end;
        self.high_water = self.high_water.max(end);
        Ok(())
    }

    /// Read_Bin_String copies length+1 bytes, including the terminating NUL.
    pub fn read_string(&mut self) -> Result<Vec<u8>, ProfileError> {
        if self.writable || self.position >= self.bytes.len() {
            return Err(ProfileError::ReadStringOutOfBounds {
                position: self.position, encoded_length: 0, capacity: self.bytes.len(),
            });
        }
        let length = self.bytes[self.position] as usize;
        let end = self.position.checked_add(length + 2)
            .ok_or(ProfileError::ReadStringOutOfBounds {
                position: self.position, encoded_length: length, capacity: self.bytes.len(),
            })?;
        if end > self.bytes.len() {
            return Err(ProfileError::ReadStringOutOfBounds {
                position: self.position, encoded_length: length, capacity: self.bytes.len(),
            });
        }
        let start = self.position + 1;
        let result = self.bytes[start..start + length + 1].to_vec();
        self.position = end;
        self.high_water = self.high_water.max(end);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_reads_are_case_insensitive_and_use_default_values() {
        let p = ProfileBuffer::new("[Sound]\r\nCard=1a2b\r\nIRQ=5\r\nDMA=1\r\n[Language]\r\nLanguage=EN\r\n");
        assert_eq!(p.get_hex("sound", "card"), 0x1a2b);
        assert_eq!(p.get_int("SOUND", "irq", 7), 5);
        assert_eq!(p.get_int("Sound", "missing", 7), 7);
        assert_eq!(p.get_hex("Sound", "missing"), 0);
    }

    #[test]
    fn profile_enumerates_keys_as_double_nul_list() {
        let p = ProfileBuffer::new("[Game]\r\nName=Alpha\r\nMode=Skirmish\r\n");
        let result = p.get_string("Game", None, None, Some(64));
        assert_eq!(&result.output[..5], b"Name\0");
        assert_eq!(&result.output[5..12], b"Mode\0\0");
        assert!(result.offset.is_some());
    }

    #[test]
    fn profile_writes_add_replace_delete_entry_and_delete_section() {
        let mut p = ProfileBuffer::new("[Game]\r\nName=Old\r\nMode=One\r\n[Other]\r\nKeep=Yes\r\n");
        p.write_string("Game", Some("Name"), Some("New"));
        assert_eq!(p.get_string("game", Some("name"), None, Some(16)).output, b"New\0");
        p.write_int("Game", Some("Count"), 12);
        assert_eq!(p.get_int("GAME", "Count", 0), 12);
        p.write_string("Game", Some("Mode"), None);
        assert_eq!(p.get_string("Game", Some("Mode"), None, Some(16)).offset, None);
        p.write_string("Other", None, None);
        assert_eq!(p.get_string("Other", Some("Keep"), None, Some(16)).offset, None);
    }

    #[test]
    fn sound_config_reader_populates_defaults_and_language() {
        let p = ProfileBuffer::new("[Sound]\r\nCard=0\r\nIRQ=0\r\nDMA=0\r\nBitsPerSample=16\r\nChannels=2\r\n[Language]\r\nLanguage=EN\r\n");
        let (config, no_sound_card) = read_private_config(&p);
        assert_eq!(config.bits_per_sample, 16);
        assert_eq!(config.channels, 2);
        assert_eq!(&config.language, b"EN\0");
        assert!(no_sound_card);
    }

    #[test]
    fn binary_read_at_exact_end_fails_but_write_at_exact_end_succeeds() {
        let mut reader = BinaryProfileBuffer::reader(&[1,2,3,4]);
        assert_eq!(reader.read_num(2).unwrap(), [1,2]);
        assert!(reader.read_num(2).is_err());
        let mut writer = BinaryProfileBuffer::writer(4);
        writer.write_num(&[1,2,3,4]).unwrap();
        assert_eq!(writer.length(), 4);
    }

    #[test]
    fn binary_string_is_length_prefixed_and_c_string_terminated() {
        let mut writer = BinaryProfileBuffer::writer(8);
        writer.write_string(b"Hi").unwrap();
        assert_eq!(writer.bytes(), &[2, b'H', b'i', 0]);
        let mut reader = BinaryProfileBuffer::reader(writer.bytes());
        assert_eq!(reader.read_string().unwrap(), b"Hi\0");
        assert_eq!(reader.position(), 4);
    }
}
