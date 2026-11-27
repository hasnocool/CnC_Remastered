//! Static TD/RA map-editor catalogs transcribed from the pinned upstream registries.
//! Source: electronicarts/CnC_Remastered_Collection @ f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae; CnCTDRAMapEditor/{TiberianDawn,RedAlert}/{BuildingTypes,SmudgeTypes}.cs.
use crate::{BuildingTypeDefinition, SmudgeTypeDefinition};

pub fn tiberian_dawn_building_types() -> Vec<BuildingTypeDefinition> {
    vec![
        BuildingTypeDefinition::from_base_mask(
            0, "weap".to_string(), "TEXT_STRUCTURE_TITLE_GDI_WEAPONS_FACTORY".to_string(), "weap".to_string(),
            vec![(0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, true, "Goodguy".to_string(),
            false, false, Some("weap2".to_string()),
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            1, "gtwr".to_string(), "TEXT_STRUCTURE_TITLE_GDI_GUARD_TOWER".to_string(), "gtwr".to_string(),
            vec![(0, 0)], 1, 1, false, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            2, "atwr".to_string(), "TEXT_STRUCTURE_TITLE_GDI_ADV_GUARD_TOWER".to_string(), "atwr".to_string(),
            vec![(0, 1)], 1, 2, false, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            3, "obli".to_string(), "TEXT_STRUCTURE_TITLE_NOD_OBELISK".to_string(), "obli".to_string(),
            vec![(0, 1)], 1, 2, false, "Badguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            4, "hq".to_string(), "TEXT_STRUCTURE_TITLE_GDI_COMM_CENTER".to_string(), "hq".to_string(),
            vec![(0, 0), (0, 1), (1, 1)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            5, "gun".to_string(), "TEXT_STRUCTURE_TITLE_NOD_TURRET".to_string(), "gun".to_string(),
            vec![(0, 0)], 1, 1, false, "Badguy".to_string(),
            false, true, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            6, "fact".to_string(), "TEXT_STRUCTURE_RA_FACT".to_string(), "fact".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)], 3, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            7, "proc".to_string(), "TEXT_STRUCTURE_TITLE_GDI_REFINERY".to_string(), "proc".to_string(),
            vec![(1, 0), (0, 1), (1, 1), (2, 1)], 3, 3, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            8, "silo".to_string(), "TEXT_STRUCTURE_TITLE_GDI_SILO".to_string(), "silo".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            9, "hpad".to_string(), "TEXT_STRUCTURE_TITLE_GDI_HELIPAD".to_string(), "hpad".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            10, "sam".to_string(), "TEXT_STRUCTURE_TITLE_NOD_SAM_SITE".to_string(), "sam".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Badguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            11, "afld".to_string(), "TEXT_STRUCTURE_TITLE_NOD_AIRFIELD".to_string(), "afld".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1), (3, 1)], 4, 2, true, "Badguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            12, "nuke".to_string(), "TEXT_STRUCTURE_TITLE_GDI_POWER_PLANT".to_string(), "nuke".to_string(),
            vec![(0, 0), (0, 1), (1, 1)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            13, "nuk2".to_string(), "TEXT_STRUCTURE_TITLE_GDI_ADV_POWER_PLANT".to_string(), "nuk2".to_string(),
            vec![(0, 0), (0, 1), (1, 1)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            14, "hosp".to_string(), "TEXT_UNIT_TITLE_HOSP".to_string(), "hosp".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            15, "pyle".to_string(), "TEXT_STRUCTURE_TITLE_GDI_BARRACKS".to_string(), "pyle".to_string(),
            vec![(0, 0), (1, 0)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            16, "arco".to_string(), "TEXT_UNIT_TITLE_ARCO".to_string(), "arco".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            17, "fix".to_string(), "TEXT_STRUCTURE_TITLE_GDI_REPAIR_FACILITY".to_string(), "fix".to_string(),
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], 3, 3, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            18, "bio".to_string(), "TEXT_UNIT_TITLE_BIO".to_string(), "bio".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            19, "hand".to_string(), "TEXT_STRUCTURE_TITLE_NOD_HAND_OF_NOD".to_string(), "hand".to_string(),
            vec![(0, 1), (1, 1), (1, 2)], 2, 3, true, "Badguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            20, "tmpl".to_string(), "TEXT_STRUCTURE_TITLE_NOD_TEMPLE_OF_NOD".to_string(), "tmpl".to_string(),
            vec![(0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, true, "Badguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            21, "eye".to_string(), "TEXT_STRUCTURE_TITLE_GDI_ADV_COMM_CENTER".to_string(), "eye".to_string(),
            vec![(0, 0), (0, 1), (1, 1)], 2, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            22, "miss".to_string(), "TEXT_STRUCTURE_TITLE_CIV35".to_string(), "miss".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)], 3, 2, true, "Goodguy".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            23, "v01".to_string(), "TEXT_STRUCTURE_TITLE_CIV1".to_string(), "v01".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            24, "v02".to_string(), "TEXT_STRUCTURE_TITLE_CIV2".to_string(), "v02".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            25, "v03".to_string(), "TEXT_STRUCTURE_TITLE_CIV3".to_string(), "v03".to_string(),
            vec![(1, 0), (0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            26, "v04".to_string(), "TEXT_STRUCTURE_TITLE_CIV4".to_string(), "v04".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            27, "v05".to_string(), "TEXT_STRUCTURE_TITLE_CIV5".to_string(), "v05".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            28, "v06".to_string(), "TEXT_STRUCTURE_TITLE_CIV6".to_string(), "v06".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            29, "v07".to_string(), "TEXT_STRUCTURE_TITLE_CIV7".to_string(), "v07".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            30, "v08".to_string(), "TEXT_STRUCTURE_TITLE_CIV8".to_string(), "v08".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            31, "v09".to_string(), "TEXT_STRUCTURE_TITLE_CIV9".to_string(), "v09".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            32, "v10".to_string(), "TEXT_STRUCTURE_TITLE_CIV10".to_string(), "v10".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            33, "v11".to_string(), "TEXT_STRUCTURE_TITLE_CIV11".to_string(), "v11".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            34, "v12".to_string(), "TEXT_STRUCTURE_TITLE_CIV12".to_string(), "v12".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            35, "v13".to_string(), "TEXT_STRUCTURE_TITLE_CIV12".to_string(), "v13".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            36, "v14".to_string(), "TEXT_STRUCTURE_TITLE_CIV13".to_string(), "v14".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            37, "v15".to_string(), "TEXT_STRUCTURE_TITLE_CIV14".to_string(), "v15".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            38, "v16".to_string(), "TEXT_STRUCTURE_TITLE_CIV15".to_string(), "v16".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            39, "v17".to_string(), "TEXT_STRUCTURE_TITLE_CIV16".to_string(), "v17".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            40, "v18".to_string(), "TEXT_STRUCTURE_TITLE_CIV17".to_string(), "v18".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Winter".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            41, "v19".to_string(), "TEXT_STRUCTURE_CIVILIAN_TITLE".to_string(), "v19".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            42, "v20".to_string(), "TEXT_STRUCTURE_TITLE_CIV18".to_string(), "v20".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            43, "v21".to_string(), "TEXT_STRUCTURE_TITLE_CIV19".to_string(), "v21".to_string(),
            vec![(0, 0), (1, 0), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            44, "v22".to_string(), "TEXT_STRUCTURE_TITLE_CIV20".to_string(), "v22".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            45, "v23".to_string(), "TEXT_STRUCTURE_TITLE_CIV21".to_string(), "v23".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            46, "v24".to_string(), "TEXT_STRUCTURE_TITLE_CIV22".to_string(), "v24".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            47, "v25".to_string(), "TEXT_STRUCTURE_TITLE_CIV1".to_string(), "v25".to_string(),
            vec![(1, 0), (0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            48, "v26".to_string(), "TEXT_STRUCTURE_TITLE_CIV23".to_string(), "v26".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            49, "v27".to_string(), "TEXT_STRUCTURE_TITLE_CIV24".to_string(), "v27".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            50, "v28".to_string(), "TEXT_STRUCTURE_TITLE_CIV25".to_string(), "v28".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            51, "v29".to_string(), "TEXT_STRUCTURE_TITLE_CIV26".to_string(), "v29".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            52, "v30".to_string(), "TEXT_STRUCTURE_TITLE_CIV27".to_string(), "v30".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            53, "v31".to_string(), "TEXT_STRUCTURE_TITLE_CIV28".to_string(), "v31".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            54, "v32".to_string(), "TEXT_STRUCTURE_TITLE_CIV29".to_string(), "v32".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            55, "v33".to_string(), "TEXT_STRUCTURE_TITLE_CIV30".to_string(), "v33".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            56, "v34".to_string(), "TEXT_STRUCTURE_TITLE_CIV31".to_string(), "v34".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            57, "v35".to_string(), "TEXT_STRUCTURE_TITLE_CIV32".to_string(), "v35".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            58, "v36".to_string(), "TEXT_STRUCTURE_TITLE_CIV33".to_string(), "v36".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            59, "v37".to_string(), "TEXT_STRUCTURE_TITLE_CIV34".to_string(), "v37".to_string(),
            vec![(1, 0), (2, 0), (3, 0), (1, 1), (2, 1), (3, 1)], 4, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Desert".to_string()])),
    ]
}

pub fn red_alert_building_types() -> Vec<BuildingTypeDefinition> {
    vec![
        BuildingTypeDefinition::from_base_mask(
            0, "atek".to_string(), "TEXT_STRUCTURE_RA_ATEK".to_string(), "atek".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            1, "iron".to_string(), "TEXT_STRUCTURE_RA_IRON".to_string(), "iron".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            2, "weap".to_string(), "TEXT_STRUCTURE_RA_WEAP".to_string(), "weap".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)], 3, 2, true, "Greece".to_string(),
            false, false, Some("weap2".to_string()),
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            3, "pdox".to_string(), "TEXT_STRUCTURE_RA_PDOX".to_string(), "pdox".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            4, "pbox".to_string(), "TEXT_STRUCTURE_RA_PBOX".to_string(), "pbox".to_string(),
            vec![(0, 0)], 1, 1, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            5, "hbox".to_string(), "TEXT_STRUCTURE_RA_HBOX".to_string(), "hbox".to_string(),
            vec![(0, 0)], 1, 1, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            6, "dome".to_string(), "TEXT_STRUCTURE_RA_DOME".to_string(), "dome".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            7, "gap".to_string(), "TEXT_STRUCTURE_RA_GAP".to_string(), "gap".to_string(),
            vec![(0, 1)], 1, 2, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            8, "gun".to_string(), "TEXT_STRUCTURE_RA_GUN".to_string(), "gun".to_string(),
            vec![(0, 0)], 1, 1, false, "Greece".to_string(),
            false, true, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            9, "agun".to_string(), "TEXT_STRUCTURE_RA_AGUN".to_string(), "agun".to_string(),
            vec![(0, 1)], 1, 2, false, "Greece".to_string(),
            false, true, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            10, "ftur".to_string(), "TEXT_STRUCTURE_RA_FTUR".to_string(), "ftur".to_string(),
            vec![(0, 0)], 1, 1, false, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            11, "fact".to_string(), "TEXT_STRUCTURE_RA_FACT".to_string(), "fact".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            12, "proc".to_string(), "TEXT_STRUCTURE_RA_PROC".to_string(), "proc".to_string(),
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (0, 2)], 3, 3, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            13, "silo".to_string(), "TEXT_STRUCTURE_RA_SILO".to_string(), "silo".to_string(),
            vec![(0, 0)], 1, 1, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            14, "hpad".to_string(), "TEXT_STRUCTURE_RA_HPAD".to_string(), "hpad".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            15, "sam".to_string(), "TEXT_STRUCTURE_RA_SAM".to_string(), "sam".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            16, "afld".to_string(), "TEXT_STRUCTURE_RA_AFLD".to_string(), "afld".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)], 3, 2, false, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            17, "powr".to_string(), "TEXT_STRUCTURE_RA_POWR".to_string(), "powr".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            18, "apwr".to_string(), "TEXT_STRUCTURE_RA_APWR".to_string(), "apwr".to_string(),
            vec![(0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            19, "stek".to_string(), "TEXT_STRUCTURE_RA_STEK".to_string(), "stek".to_string(),
            vec![(0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, true, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            20, "hosp".to_string(), "TEXT_STRUCTURE_RA_HOSP".to_string(), "hosp".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            21, "barr".to_string(), "TEXT_STRUCTURE_RA_BARR".to_string(), "barr".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            22, "tent".to_string(), "TEXT_STRUCTURE_RA_TENT".to_string(), "tent".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            23, "kenn".to_string(), "TEXT_STRUCTURE_RA_KENN".to_string(), "kenn".to_string(),
            vec![(0, 0)], 1, 1, false, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            24, "fix".to_string(), "TEXT_STRUCTURE_RA_FIX".to_string(), "fix".to_string(),
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], 3, 3, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            25, "bio".to_string(), "TEXT_STRUCTURE_RA_BIO".to_string(), "bio".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            26, "miss".to_string(), "TEXT_STRUCTURE_RA_MISS".to_string(), "miss".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)], 3, 2, true, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            27, "syrd".to_string(), "TEXT_STRUCTURE_RA_SYRD".to_string(), "syrd".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            28, "spen".to_string(), "TEXT_STRUCTURE_RA_SPEN".to_string(), "spen".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, false, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            29, "mslo".to_string(), "TEXT_STRUCTURE_RA_MSLO".to_string(), "mslo".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            30, "fcom".to_string(), "TEXT_STRUCTURE_RA_FCOM".to_string(), "fcom".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, true, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            31, "tsla".to_string(), "TEXT_STRUCTURE_RA_TSLA".to_string(), "tsla".to_string(),
            vec![(0, 1)], 1, 2, false, "USSR".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            32, "weap".to_string(), "TEXT_STRUCTURE_RA_WEAF".to_string(), "weap".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)], 3, 2, true, "Greece".to_string(),
            true, false, Some("weap2".to_string()),
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            33, "fact".to_string(), "TEXT_STRUCTURE_RA_FACF".to_string(), "fact".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, true, "Greece".to_string(),
            true, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            34, "syrd".to_string(), "TEXT_STRUCTURE_RA_SYRF".to_string(), "syrd".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, false, "Greece".to_string(),
            true, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            35, "spen".to_string(), "TEXT_STRUCTURE_RA_SPEF".to_string(), "spen".to_string(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)], 3, 3, false, "USSR".to_string(),
            true, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            36, "dome".to_string(), "TEXT_STRUCTURE_RA_DOMF".to_string(), "dome".to_string(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2, 2, true, "Greece".to_string(),
            true, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            43, "minv".to_string(), "TEXT_STRUCTURE_RA_MINV".to_string(), "minv".to_string(),
            vec![(0, 0)], 1, 1, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            44, "minp".to_string(), "TEXT_STRUCTURE_RA_MINP".to_string(), "minp".to_string(),
            vec![(0, 0)], 1, 1, false, "Greece".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            45, "v01".to_string(), "TEXT_STRUCTURE_TITLE_CIV1".to_string(), "v01".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            46, "v02".to_string(), "TEXT_STRUCTURE_TITLE_CIV2".to_string(), "v02".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            47, "v03".to_string(), "TEXT_STRUCTURE_TITLE_CIV3".to_string(), "v03".to_string(),
            vec![(1, 0), (0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            48, "v04".to_string(), "TEXT_STRUCTURE_TITLE_CIV4".to_string(), "v04".to_string(),
            vec![(0, 1), (1, 1)], 2, 2, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            49, "v05".to_string(), "TEXT_STRUCTURE_TITLE_CIV5".to_string(), "v05".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            50, "v06".to_string(), "TEXT_STRUCTURE_TITLE_CIV6".to_string(), "v06".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            51, "v07".to_string(), "TEXT_STRUCTURE_TITLE_CIV7".to_string(), "v07".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            52, "v08".to_string(), "TEXT_STRUCTURE_TITLE_CIV8".to_string(), "v08".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            53, "v09".to_string(), "TEXT_STRUCTURE_TITLE_CIV9".to_string(), "v09".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            54, "v10".to_string(), "TEXT_STRUCTURE_TITLE_CIV10".to_string(), "v10".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            55, "v11".to_string(), "TEXT_STRUCTURE_TITLE_CIV11".to_string(), "v11".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            56, "v12".to_string(), "TEXT_STRUCTURE_RA_CIVILIAN".to_string(), "v12".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            57, "v13".to_string(), "TEXT_STRUCTURE_RA_CIVILIAN".to_string(), "v13".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            58, "v14".to_string(), "TEXT_STRUCTURE_TITLE_CIV13".to_string(), "v14".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            59, "v15".to_string(), "TEXT_STRUCTURE_TITLE_CIV14".to_string(), "v15".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            60, "v16".to_string(), "TEXT_STRUCTURE_TITLE_CIV15".to_string(), "v16".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            61, "v17".to_string(), "TEXT_STRUCTURE_TITLE_CIV16".to_string(), "v17".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            62, "v18".to_string(), "TEXT_STRUCTURE_TITLE_CIV17".to_string(), "v18".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            63, "v19".to_string(), "TEXT_STRUCTURE_RA_CIVILIAN".to_string(), "v19".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(Some(vec!["Temperate".to_string(), "Snow".to_string()])),
        BuildingTypeDefinition::from_base_mask(
            82, "barl".to_string(), "TEXT_STRUCTURE_RA_BARL".to_string(), "barl".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            83, "brl3".to_string(), "TEXT_STRUCTURE_RA_BRL3".to_string(), "brl3".to_string(),
            vec![(0, 0)], 1, 1, false, "Neutral".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            84, "quee".to_string(), "TEXT_STRUCTURE_RA_QUEE".to_string(), "quee".to_string(),
            vec![(0, 0), (1, 0)], 2, 1, false, "Special".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            85, "lar1".to_string(), "TEXT_STRUCTURE_RA_LAR1".to_string(), "lar1".to_string(),
            vec![(0, 0)], 1, 1, false, "Special".to_string(),
            false, false, None,
        ).with_theaters(None),
        BuildingTypeDefinition::from_base_mask(
            86, "lar2".to_string(), "TEXT_STRUCTURE_RA_LAR2".to_string(), "lar2".to_string(),
            vec![(0, 0)], 1, 1, false, "Special".to_string(),
            false, false, None,
        ).with_theaters(None),
    ]
}

pub fn tiberian_dawn_smudge_types() -> Vec<SmudgeTypeDefinition> {
    (0..12).map(|id| SmudgeTypeDefinition { id, name: ["cr1","cr2","cr3","cr4","cr5","cr6","sc1","sc2","sc3","sc4","sc5","sc6"][id as usize].to_string(), flags: 0, width: 1, height: 1 }).chain([
        SmudgeTypeDefinition { id: 12, name: "bib1".into(), flags: 3, width: 4, height: 2 },
        SmudgeTypeDefinition { id: 13, name: "bib2".into(), flags: 5, width: 3, height: 2 },
        SmudgeTypeDefinition { id: 14, name: "bib3".into(), flags: 9, width: 2, height: 2 },
    ]).collect()
}

pub fn red_alert_smudge_types() -> Vec<SmudgeTypeDefinition> {
    (0..12).map(|id| SmudgeTypeDefinition { id, name: ["cr1","cr2","cr3","cr4","cr5","cr6","sc1","sc2","sc3","sc4","sc5","sc6"][id as usize].to_string(), flags: 0, width: 1, height: 1 }).chain([
        SmudgeTypeDefinition { id: 12, name: "bib1".into(), flags: 3, width: 4, height: 2 },
        SmudgeTypeDefinition { id: 13, name: "bib2".into(), flags: 5, width: 3, height: 2 },
        SmudgeTypeDefinition { id: 14, name: "bib3".into(), flags: 9, width: 2, height: 2 },
    ]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn catalog_counts_and_ids_match_upstream_registries() {
        let td=tiberian_dawn_building_types(); let ra=red_alert_building_types();
        assert_eq!(td.len(), 60); assert_eq!(ra.len(), 63);
        assert_eq!(tiberian_dawn_smudge_types().len(), 15);
        assert_eq!(red_alert_smudge_types().len(), 15);
        assert!(td.iter().any(|v| v.id == 0 && v.name == "weap"));
        assert!(ra.iter().any(|v| v.id == 0 && v.name == "atek"));
    }
}
