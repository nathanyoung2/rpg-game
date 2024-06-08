use rpg_game::entity::Entity;
use rpg_game::moves::Move;

fn main() {
    let mut player = Entity::new(200, 20, 20, 20);
    player.set_moves(vec![Move::Attack, Move::Heal]);

    let mut enemy = Entity::new(200, 20, 20, 20);
}
