extern crate winapi;

mod offsets;

use std::char::from_u32;
use winapi::shared::minwindef::{BYTE, DWORD, FALSE, HMODULE, LPVOID};
use winapi::shared::basetsd::DWORD_PTR;
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::psapi::{EnumProcesses, EnumProcessModulesEx, GetModuleFileNameExW, GetProcessImageFileNameW, LIST_MODULES_32BIT, LIST_MODULES_64BIT};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE32};
use winapi::um::tlhelp32::{TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS};

use std::time;
use std::mem::{MaybeUninit, size_of, size_of_val};
use std::process::exit;
use std::string::String;
use std::thread::sleep;
use winapi::ctypes::{c_void, wchar_t};
use winapi::um::tlhelp32::{Process32First, Process32Next, PROCESSENTRY32};
use crate::modules::*;

use crate::player::Player;
use crate::process::Process;

mod memoryreader;
mod player;
mod process;
mod modules;

fn main() {


    let mut vec:Vec<&dyn Module> = Vec::new();

    let mut proc = Process::new("ac_client.exe".to_string());
    let mut local_player = Player::new(&mut proc, offsets::OFFSET_LOCAL_PLAYER);
    let godmode = Godmode {
        player: &mut local_player,
    };
    let ammo = Ammo{
      player: &mut local_player,
    };
    let armor = Armor{
        player: &mut local_player,
    };
    let he = He{
        player: &mut local_player,
    };
    vec.push(&godmode);
    vec.push(&ammo);
    vec.push(&armor);
    vec.push(&he);

    loop {
        for mo in vec.iter() {
            mo.tick();
        }
        let time = time::Duration::from_millis(1);
        sleep(time);
    }
}
