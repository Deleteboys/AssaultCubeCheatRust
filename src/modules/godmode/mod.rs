use std::thread::sleep;
use std::time;
use crate::modules::Module;
use crate::Player;

pub struct Godmode {
    pub player: *mut Player,
}

impl Module for Godmode {

    fn tick(&self) {
        unsafe { (*self.player).set_health(9999); }
        // todo!()
    }

    // pub fn new(player: Player) -> Godmode {
    //     Godmode {
    //         module: Module {
    //             player,
    //         }
    //     }
    // }
    //
    // pub fn heal_loop(&self) {
    //     loop {
    //         self.module.player.set_health(9999);
    //         let millis = time::Duration::from_millis(1);
    //         sleep(millis);
    //     }
    // }
}