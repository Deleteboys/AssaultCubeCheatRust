use crate::{Module, Player};

pub struct Armor {
    pub player: *mut Player,
}

impl Module for Armor {

    fn tick(&self) {
        unsafe {(*self.player).set_armor(9999);}
    }
}