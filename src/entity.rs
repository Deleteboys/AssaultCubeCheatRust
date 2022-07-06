use crate::Player;

pub struct Entity {
    pub player:Player,
}

impl Entity {

    pub fn new(player:Player) -> Entity {
        Entity{player}
    }

}