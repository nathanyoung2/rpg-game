pub mod player;

pub trait Status {
    fn inflict(&self, target: dyn Character);
}

pub trait Move {
    fn execute(&self, target: dyn Character);
}

pub trait Character {
    fn take_damage(&mut self, damage: u32);
    fn change_stats(&mut self, stat: Stat, amount: u32, subtract: bool);
}

pub enum Stat {
    Attack,
    Defense,
    Accuracy,
    MaxHealth,
}
