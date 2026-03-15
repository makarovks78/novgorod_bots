#[derive(Debug)]
pub struct Future {
    pub wood: u8,
    pub food: u8,
    pub metal: u8,
    pub weapon: u8,
    pub wax: u8,
    pub wool: u8,
    pub money: u8,
    pub people: u8,
    pub vp: u8,
    pub reputation: u8,
    pub action_cards: u8,
    pub law_cards: u8,
    pub war_cards: u8,
}

impl Future {
    pub fn new() -> Future {
        Future {
            wood: 0,
            food: 0,
            metal: 0,
            weapon: 0,
            wax: 0,
            wool: 0,
            money: 0,
            people: 0,
            vp: 0,
            reputation: 0,
            action_cards: 0,
            law_cards: 0,
            war_cards: 0,
        }
    }
}
