use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct InputResources {
    #[serde(default)]
    wood: u8,
    #[serde(default)]
    food: u8,
    #[serde(default)]
    metal: u8,
    #[serde(default)]
    weapon: u8,
    #[serde(default)]
    wax: u8,
    #[serde(default)]
    wool: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum WorkshopTypeEnum {
    Weapons,
    Feeds,
    Waxes,
    Wools,
    Church,
    School,
}

impl WorkshopTypeEnum {
    pub fn name(&self) -> &str {
        match self {
            WorkshopTypeEnum::Weapons => "Кузня",
            WorkshopTypeEnum::Feeds => "Едальня",
            WorkshopTypeEnum::Waxes => "Воск",
            WorkshopTypeEnum::Wools => "Пушнина",
            WorkshopTypeEnum::Church => "Церковь",
            WorkshopTypeEnum::School => "Школа",
        }
    }
}
