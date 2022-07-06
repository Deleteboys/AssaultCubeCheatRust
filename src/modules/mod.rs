use crate::Player;
mod godmode;
mod ammo;
mod armor;
mod he;
mod aimbot;

pub use godmode::*;
pub use ammo::*;
pub use armor::*;
pub use he::*;
pub use aimbot::*;

pub trait Module {
    // player: Player,
    fn tick(&self);

    // fn new(&self, player: *mut Player) -> self;
}