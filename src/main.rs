extern crate winapi;

mod offsets;

use std::borrow::BorrowMut;
use std::char::from_u32;
use std::fmt::format;
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
use crate::entity::Entity;
use crate::modules::*;

use crate::player::Player;
use crate::process::Process;

mod memoryreader;
mod player;
mod process;
mod modules;
mod entity;


pub struct mut entity_list: Vec<Entity> = Vec::new();

fn main() {
    let mut vec: Vec<&dyn Module> = Vec::new();

    let mut proc = Process::new("ac_client.exe".to_string());
    let mut local_player = Player::new(&mut proc, offsets::OFFSET_LOCAL_PLAYER);
    let godmode = Godmode {
        player: &mut local_player,
    };
    let ammo = Ammo {
        player: &mut local_player,
    };
    let armor = Armor {
        player: &mut local_player,
    };
    let he = He {
        player: &mut local_player,
    };
    let aimbot = Aimbot {
        player: &mut local_player,
    };
    vec.push(&godmode);
    vec.push(&ammo);
    vec.push(&armor);
    vec.push(&he);
    vec.push(&aimbot);

    // let mut vec2 = Vec::new();
    // vec2.push(78);
    // vec2.push(0x88);
    // vec2.push(0xf8);
    // memoryreader::read_mem_with_offsets(proc.process_handle,0x0,vec2);

    loop {
        &entity_list.clear();
        for i in 1..=32 {
            let player_name = memoryreader::read_mem_with_offsets(proc.process_handle, proc.game_id, 0x10F4F8, vec![i * 4, 0x225]);
            if player_name == 0 {
                break;
            }
            let entity_offset = memoryreader::read_mem_with_offsets(proc.process_handle, proc.game_id, 0x10F4F8, vec![i * 4]);
            let entity = Entity::new(Player::new(&mut proc, entity_offset));
            entity_list.push(entity);
            // let player_health = memoryreader::read_mem_with_offsets(proc.process_handle, proc.game_id, 0x10F4F8, vec![i * 4, 0xf8]);
            // println!("----User: {} ----", player_name.to_string());
            // println!("  Health: {player_health}");
            // dbg!(entity_list.len());
        }


        for mo in vec.iter() {
            mo.tick();
        }
        let time = time::Duration::from_millis(1);
        sleep(time);
    }
}

