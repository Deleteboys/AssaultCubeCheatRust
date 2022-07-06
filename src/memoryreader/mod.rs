use winapi::shared::minwindef::{BYTE, DWORD, FALSE, HMODULE, LPVOID};
use winapi::shared::basetsd::DWORD_PTR;
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::psapi::{EnumProcesses, EnumProcessModulesEx, GetModuleFileNameExW, GetProcessImageFileNameW, LIST_MODULES_32BIT, LIST_MODULES_64BIT};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE32};
use winapi::um::tlhelp32::{TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS};
use winapi::ctypes::{c_void, wchar_t};
use winapi::um::tlhelp32::{Process32First, Process32Next, PROCESSENTRY32};

use std::time;
use std::char::from_u32;
use std::mem::{MaybeUninit, size_of, size_of_val};
use std::process::exit;
use std::string::String;
use std::thread::sleep;

pub fn get_prod_id(proc_name: String) -> u32 {
    let mut proc_id = 0;
    let h_snapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if h_snapshot != INVALID_HANDLE_VALUE {
        let mut proc_entry = PROCESSENTRY32 {
            dwSize: size_of::<PROCESSENTRY32>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; 260],
        };
        unsafe {
            if Process32First(h_snapshot, &mut proc_entry) != 0 {
                loop {
                    let mut test = String::new();
                    for x in proc_entry.szExeFile {
                        test.push(from_u32(x as u32).unwrap());
                    }
                    if test.contains(&proc_name) {
                        proc_id = proc_entry.th32ProcessID;
                        break;
                    }
                    if Process32Next(h_snapshot, &mut proc_entry) == 0 {
                        break;
                    }
                }
            }
        }
        unsafe { CloseHandle(h_snapshot) };
    }
    return proc_id;
}

pub fn get_module_base_address(proc_id: DWORD, mod_name: String) -> u32 {
    let mut mod_base_addr: u32 = 0;
    let h_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, proc_id) };
    if h_snap != INVALID_HANDLE_VALUE {
        let mut module_entry = MaybeUninit::<MODULEENTRY32>::uninit();
        unsafe {
            module_entry.assume_init_mut().dwSize = size_of::<MODULEENTRY32>() as u32;
            if Module32First(h_snap, module_entry.assume_init_mut()) != 0 {
                loop {
                    let mut test = String::new();
                    for x in module_entry.assume_init().szModule {
                        test.push(from_u32(x as u8 as u32).unwrap());
                    }
                    if test.contains(&mod_name) {
                        mod_base_addr = module_entry.assume_init().modBaseAddr as u32;
                        break;
                    }
                    if Module32Next(h_snap, module_entry.assume_init_mut()) == 0 {
                        break;
                    }
                }
            }
        }
    }
    return mod_base_addr;
}

pub fn get_process_as_handel(process_id: u32) -> HANDLE {
    let mut process_handel;
    unsafe {
        process_handel = OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id);
    }
    return process_handel;
}

pub fn read_mem_with_offsets(h_process: HANDLE, base_address: u32, address: u32, offsets: Vec<u32>) -> u32 {
    let mut value: u32 = 0;
    let mut temp = read_mem(h_process, base_address + address);
    for i in 0..offsets.len() - 1 {
        temp = read_mem(h_process, temp + offsets[i as usize]);
    }
    value = read_mem(h_process,temp+offsets[(offsets.len()-1) as usize]);
    value
}

pub fn read_mem(h_process: HANDLE, dw_addr: DWORD) -> u32 {
    let mut value: u32 = 0;
    unsafe {
        ReadProcessMemory(h_process, dw_addr as usize as *mut _, &mut value as *mut _ as LPVOID, size_of::<u32>(), 0 as *mut usize);
    };
    return value;
}

pub fn write_mem<T>(h_process: HANDLE, dw_addr: DWORD, mut value: T) {
    unsafe { WriteProcessMemory(h_process, dw_addr as usize as *mut _, &mut value as *mut _ as LPVOID, size_of::<T>(), 0 as *mut usize); };
}