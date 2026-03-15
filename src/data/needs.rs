#[derive(Debug, Clone)]
pub struct Needs {
    pub current: NeedsResoures,
    pub last: NeedsResoures,
}

#[derive(Debug, Clone)]
pub struct NeedsResoures {
    pub wood: u8,
    pub food: u8,
    pub metal: u8,
    pub weapon: u8,
    pub wax: u8,
    pub wool: u8,
    pub money: u8,
    pub people: u8,
    pub reputation: u8,
    pub action_cards: u8,
    pub law_cards: u8,
    pub war_cards: u8,
}

impl NeedsResoures {
    pub fn new() -> NeedsResoures {
        NeedsResoures {
            wood: 0,
            food: 0,
            metal: 0,
            weapon: 0,
            wax: 0,
            wool: 0,
            money: 0,
            people: 0,
            reputation: 0,
            action_cards: 0,
            law_cards: 0,
            war_cards: 0,
        }
    }

    pub fn reset(&mut self) {
        self.wood = 0;
        self.food = 0;
        self.metal = 0;
        self.weapon = 0;
        self.wax = 0;
        self.wool = 0;
        self.money = 0;
        self.people = 0;
        self.reputation = 0;
        self.action_cards = 0;
        self.law_cards = 0;
        self.war_cards = 0;
    }
}

impl Needs {
    pub fn new() -> Needs {
        Needs {
            current: NeedsResoures::new(),
            last: NeedsResoures::new(),
        }
    }
    pub fn move_current_to_last(&mut self) {
        self.last.wood = self.current.wood;
        self.last.food = self.current.food;
        self.last.metal = self.current.metal;
        self.last.weapon = self.current.weapon;
        self.last.wax = self.current.wax;
        self.last.wool = self.current.wool;
        self.last.money = self.current.money;
        self.last.people = self.current.people;
        self.last.reputation = self.current.reputation;
        self.last.action_cards = self.current.action_cards;
        self.last.law_cards = self.current.law_cards;
        self.last.war_cards = self.current.war_cards;
        self.current.reset();
    }

    pub fn put_wood(&mut self, qty: u8) {
        if self.current.wood < qty {
            self.current.wood = qty 
        }
    }
    pub fn put_food(&mut self, qty: u8) {
        if self.current.food < qty {
            self.current.food = qty 
        }
    }
    pub fn put_metal(&mut self, qty: u8) {
        if self.current.metal < qty {
            self.current.metal = qty 
        }
    }
    pub fn put_weapon(&mut self, qty: u8) {
        if self.current.weapon < qty {
            self.current.weapon = qty 
        }
    }
    pub fn put_wax(&mut self, qty: u8) {
        if self.current.wax < qty {
            self.current.wax = qty 
        }
    }
    pub fn put_wool(&mut self, qty: u8) {
        if self.current.wool < qty {
            self.current.wool = qty 
        }
    }
    pub fn put_money(&mut self, qty: u8) {
        if self.current.money < qty {
            self.current.money = qty 
        }
    }
    pub fn put_people(&mut self, qty: u8) {
        if self.current.people < qty {
            self.current.people = qty 
        }
    }
    pub fn put_reputation(&mut self, qty: u8) {
        if self.current.reputation < qty {
            self.current.reputation = qty 
        }
    }
    pub fn put_action_cards(&mut self, qty: u8) {
        if self.current.action_cards < qty {
            self.current.action_cards = qty 
        }
    }
    pub fn put_law_cards(&mut self, qty: u8) {
        if self.current.law_cards < qty {
            self.current.law_cards = qty 
        }
    }
    pub fn put_war_cards(&mut self, qty: u8) {
        if self.current.war_cards < qty {
            self.current.war_cards = qty 
        }
    }
}
