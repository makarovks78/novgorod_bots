use crate::{
    data::month_actions::{
        month_action_event, month_action_feed, month_action_food, month_action_market,
        month_action_people, month_action_vote, month_action_war, month_action_wood,
        month_action_workshop,
    },
    game::Game,
};

#[derive(Debug, Clone)]
pub enum MonthActionEnum {
    War,
    Event,
    Food,
    Wood,
    Feed,
    Vote,
    Market,
    Workshop,
    People,
}

pub enum MonthEnum {
    April(Vec<MonthActionEnum>),
    May(Vec<MonthActionEnum>),
    June(Vec<MonthActionEnum>),
    July(Vec<MonthActionEnum>),
    August(Vec<MonthActionEnum>),
    September(Vec<MonthActionEnum>),
    October(Vec<MonthActionEnum>),
    November(Vec<MonthActionEnum>),
    December(Vec<MonthActionEnum>),
    January(Vec<MonthActionEnum>),
    February(Vec<MonthActionEnum>),
    March(Vec<MonthActionEnum>),
}

impl MonthEnum {
    pub fn actions(&self) -> &Vec<MonthActionEnum> {
        match self {
            MonthEnum::April(actions) => actions,
            MonthEnum::May(actions) => actions,
            MonthEnum::June(actions) => actions,
            MonthEnum::July(actions) => actions,
            MonthEnum::August(actions) => actions,
            MonthEnum::September(actions) => actions,
            MonthEnum::October(actions) => actions,
            MonthEnum::November(actions) => actions,
            MonthEnum::December(actions) => actions,
            MonthEnum::January(actions) => actions,
            MonthEnum::February(actions) => actions,
            MonthEnum::March(actions) => actions,
        }
    }
}

pub struct Month {
    pub current_index: u8,
    pub max_count: u8,
    pub current_count: u8,
    pub months: Vec<MonthEnum>,
}

impl Month {
    pub fn new(max_count: u8) -> Month {
        Month {
            current_index: 1,
            max_count: max_count,
            current_count: 1,
            months: vec![
                MonthEnum::April(vec![
                    MonthActionEnum::War,
                    MonthActionEnum::Market,
                    MonthActionEnum::Workshop,
                ]),
                MonthEnum::May(vec![
                    MonthActionEnum::Event,
                    MonthActionEnum::People,
                    MonthActionEnum::Vote,
                ]),
                MonthEnum::June(vec![
                    MonthActionEnum::War,
                    MonthActionEnum::Food,
                    MonthActionEnum::Workshop,
                    MonthActionEnum::Wood,
                ]),
                MonthEnum::July(vec![
                    MonthActionEnum::Event,
                    MonthActionEnum::Market,
                    MonthActionEnum::Feed,
                ]),
                MonthEnum::August(vec![
                    MonthActionEnum::War,
                    MonthActionEnum::People,
                    MonthActionEnum::Workshop,
                    MonthActionEnum::Wood,
                ]),
                MonthEnum::September(vec![
                    MonthActionEnum::Event,
                    MonthActionEnum::Food,
                    MonthActionEnum::Vote,
                ]),
                MonthEnum::October(vec![
                    MonthActionEnum::War,
                    MonthActionEnum::Market,
                    MonthActionEnum::Workshop,
                    MonthActionEnum::Food,
                ]),
                MonthEnum::November(vec![
                    MonthActionEnum::Event,
                    MonthActionEnum::Feed,
                    MonthActionEnum::Vote,
                ]),
                MonthEnum::December(vec![
                    MonthActionEnum::War,
                    MonthActionEnum::People,
                    MonthActionEnum::Workshop,
                    MonthActionEnum::Wood,
                ]),
                MonthEnum::January(vec![
                    MonthActionEnum::Event,
                    MonthActionEnum::Market,
                    MonthActionEnum::Vote,
                ]),
                MonthEnum::February(vec![
                    MonthActionEnum::War,
                    MonthActionEnum::Feed,
                    MonthActionEnum::Workshop,
                    MonthActionEnum::Wood,
                ]),
                MonthEnum::March(vec![
                    MonthActionEnum::Event,
                    MonthActionEnum::People,
                    MonthActionEnum::Vote,
                ]),
            ],
        }
    }

    pub fn next(&mut self) -> bool {
        if self.current_count < self.max_count {
            self.current_index += 1;
            if self.current_index > 12 {
                self.current_index = 1;
            }
            self.current_count += 1;
            true
        } else {
            false
        }
    }

    pub fn get_current_month(&self) -> &MonthEnum {
        &self.months[(self.current_index - 1) as usize]
    }

    pub fn get_current_month_name(&self) -> &str {
        match self.get_current_month() {
            MonthEnum::April(_) => "Апрель",
            MonthEnum::May(_) => "Май",
            MonthEnum::June(_) => "Июню",
            MonthEnum::July(_) => "Июль",
            MonthEnum::August(_) => "Агуст",
            MonthEnum::September(_) => "Сентябрь",
            MonthEnum::October(_) => "Октябрь",
            MonthEnum::November(_) => "Ноябрь",
            MonthEnum::December(_) => "Декабрь",
            MonthEnum::January(_) => "Январь",
            MonthEnum::February(_) => "Февраль",
            MonthEnum::March(_) => "Март",
        }
    }

    pub fn process_current_month_actions(&self, game: &mut Game) {
        let actions: Vec<MonthActionEnum> =
            self.get_current_month().actions().iter().cloned().collect();
        for action in actions {
            match action {
                MonthActionEnum::War => month_action_war(game),
                MonthActionEnum::Event => month_action_event(game),
                MonthActionEnum::Food => month_action_food(game),
                MonthActionEnum::Wood => month_action_wood(game),
                MonthActionEnum::Feed => month_action_feed(game),
                MonthActionEnum::Vote => month_action_vote(game),
                MonthActionEnum::Market => month_action_market(game),
                MonthActionEnum::Workshop => month_action_workshop(game),
                MonthActionEnum::People => month_action_people(game),
            }
        }
    }
}
