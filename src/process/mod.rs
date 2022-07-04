use winapi::ctypes::c_void;
use winapi::um::winnt::HANDLE;
use crate::offsets;
use crate::memoryreader;

pub struct Process {
    pub process_id: u32,
    pub process_handle: HANDLE,
    pub process_name: String,
    pub game_id: u32,
}

impl Process {

    pub fn set_all(&mut self) {
        self.process_id = memoryreader::get_prod_id(self.process_name.to_string());
        self.process_handle = memoryreader::get_process_as_handel(self.process_id);
        self.game_id = memoryreader::get_module_base_address(self.process_id,self.process_name.to_string());
    }

    pub fn new(process_name: String) -> Process {
        let mut proc = Process {
            process_name,
            process_id: 0,
            process_handle: 0 as HANDLE,
            game_id: 0,
        };
        proc.set_all();
        proc
    }
}