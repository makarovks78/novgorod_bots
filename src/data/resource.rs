#[derive(Debug, Clone)]
pub enum CubeResourceTypeEnum {
    Wood,
    Food,
    Metal,
    Weapon,
    Wax,
    Wool,
}

impl CubeResourceTypeEnum {
    pub fn name(&self) -> &str {
        match self {
            CubeResourceTypeEnum::Wood => "Дерево",
            CubeResourceTypeEnum::Food => "Еда",
            CubeResourceTypeEnum::Metal => "Метал",
            CubeResourceTypeEnum::Weapon => "Оружие",
            CubeResourceTypeEnum::Wax => "Воск",
            CubeResourceTypeEnum::Wool => "Пушнина",
        }
    }
}

#[derive(Debug)]
pub enum CommonIndicatorEnum {
    Reputation,
    People,
}

#[derive(Debug)]
pub enum HandResourceTypeEnum {
    Money,
    VictoryPoints,
}
