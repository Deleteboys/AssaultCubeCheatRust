use winapi::um::winnt::HANDLE;
use crate::player::offsets_loader::OffsetsLoader;
use crate::Process;
use crate::memoryreader;
use crate::offsets;

mod offsets_loader;

pub struct Player {
    offsets: OffsetsLoader,
    process: *mut Process,
}

impl Player {
    pub fn new(proc: &mut Process, player_offset: u32) -> Player {
        Player {
            offsets: OffsetsLoader::new(player_offset, proc),
            process: proc,
        }
    }

    pub fn get_health(&self) -> u32 {
        unsafe { memoryreader::read_mem((*self.process).process_handle, self.offsets.player_pointer + offsets::M_HEALTH) }
    }

    pub fn set_health(&self, value:u32) {
        unsafe {memoryreader::write_mem((*self.process).process_handle, self.offsets.player_pointer+offsets::M_HEALTH,value)}
    }

    pub fn set_ammo(&self, value:u32) {
        unsafe {memoryreader::write_mem((*self.process).process_handle, self.offsets.player_pointer+offsets::M_SEC_AMMO,value)}
    }

    pub fn set_armor(&self, value:u32) {
        unsafe {memoryreader::write_mem((*self.process).process_handle, self.offsets.player_pointer+offsets::M_VEST,value)}
    }

    pub fn set_he(&self, value:u32) {
        unsafe {memoryreader::write_mem((*self.process).process_handle, self.offsets.player_pointer+offsets::M_FLASHBANGS,value)}
    }

}