use crate::{entity_list, main, Module, Player};

pub struct Aimbot {
    pub player: *mut Player,
}

impl Module for Aimbot {
    fn tick(&self) {
            for x in entity_list {
                println!("{}", x.player.get_health());
            }
    }

    // fn new(player: *mut Player) -> Aimbot {
    //     Aimbot { player }
    // }
}