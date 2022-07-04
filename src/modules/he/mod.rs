use crate::{Module, Player};

pub struct He {
    pub player: *mut Player,
}

impl Module for He {
    fn tick(&self) {
        unsafe {(*self.player).set_he(9999);}
    }
}