use winapi::um::winnt::HANDLE;
use crate::{memoryreader, Process};
use crate::offsets;

pub struct OffsetsLoader {
    player_offset: u32,
    process: *mut Process,
    pub player_pointer: u32,
}

impl OffsetsLoader {

    pub  fn calculate_player_pointer(&mut self) {
        unsafe { self.player_pointer = memoryreader::read_mem((*self.process).process_handle, (*self.process).game_id + self.player_offset); }
    }

    pub fn new(player_offset: u32, process: &mut Process) -> OffsetsLoader {
        let mut offsets = OffsetsLoader {
            player_offset,
            process,
            player_pointer: 0,
        };
        offsets.calculate_player_pointer();
        offsets
    }
}