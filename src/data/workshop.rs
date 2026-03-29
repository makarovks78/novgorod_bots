use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct InputResources {
    #[serde(default)]
    pub wood: u8,
    #[serde(default)]
    pub food: u8,
    #[serde(default)]
    pub metal: u8,
    #[serde(default)]
    pub weapon: u8,
    #[serde(default)]
    pub wax: u8,
    #[serde(default)]
    pub wool: u8,
}

impl InputResources {
    /// Возвращает суммарное количество ресурсов
    pub fn total(&self) -> u8 {
        self.wood + self.food + self.metal + self.weapon + self.wax + self.wool
    }

    /// Пуста ли комбинация ресурсов
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }
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
