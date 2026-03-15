use crate::game::Game;
use log::info;

pub fn month_action_war(game: &mut Game) {
    info!("Выполняется действие месяца: война");
}
pub fn month_action_event(game: &mut Game) {
    info!("Выполняется действие месяца: событие");
}
pub fn month_action_food(game: &mut Game) {
    info!("Выполняется действие месяца: еда");
}
pub fn month_action_wood(game: &mut Game) {
    info!("Выполняется действие месяца: дерево");
}
pub fn month_action_feed(game: &mut Game) {
    info!("Выполняется действие месяца: прокорм");
}
pub fn month_action_vote(game: &mut Game) {
    info!("Выполняется действие месяца: вече");
}
pub fn month_action_market(game: &mut Game) {
    info!("Выполняется действие месяца: рынок");
}
pub fn month_action_workshop(game: &mut Game) {
    info!("Выполняется действие месяца: мастерские");
}
pub fn month_action_people(game: &mut Game) {
    info!("Выполняется действие месяца: люди");

    for player_hash in game.get_players_hashes() {
        let player = game.get_player_by_hash(player_hash);
        game.give_people(
            player_hash,
            game.indexes
                .reputation_people_map
                .get(&player.reputation)
                .unwrap()
                .clone(),
        );
    }
}


