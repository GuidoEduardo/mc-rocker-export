use std::ffi::c_void;

use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};

use winapi::shared::minwindef::FALSE;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

fn main() {
    println!("Finding process");

    let system = System::new_all();
    let buffer = b"Hello, World!\0";
    let buffer_size = buffer.len();
    let mut bytes_written = 0;

    let processes_map = system.processes_by_name("Minecraft.Windows.exe");
    let processes: Vec<&Process> = processes_map.into_iter().collect();

    let desired_access = PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE;

    let h_process = unsafe {
        OpenProcess(
            desired_access,
            FALSE,
            processes.first().unwrap().pid().as_u32(),
        )
    };

    println!("Processo aberto com sucesso.");

    let p1 = unsafe {
        VirtualAllocEx(
            h_process,
            std::ptr::null_mut(),
            (buffer_size + 1) as usize,
            12288 as u32,
            64 as u32,
        )
    };

    unsafe {
        WriteProcessMemory(
            h_process,
            p1,
            buffer.as_ptr() as *const c_void,
            buffer_size,
            &mut bytes_written,
        )
    };

    let error_code = unsafe { GetLastError() };

    println!("Error: {}", error_code);
    println!("{}", bytes_written);
}
