use crate::Player;
mod godmode;
mod ammo;
mod armor;
mod he;
pub use godmode::*;
pub use ammo::*;
pub use armor::*;
pub use he::*;

pub trait Module {
    // player: Player,
    fn tick(&self);
}