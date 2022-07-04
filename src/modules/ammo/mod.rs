use crate::{Module, Player};

pub struct Ammo{
    pub player: *mut Player,
}

impl Module for Ammo {

    fn tick(&self) {
        unsafe { (*self.player).set_ammo(9999); }
    }
}