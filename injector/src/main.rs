use std::ffi::c_void;

use log::{error, info};
use std::ptr::null_mut as NULL;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};

use injector::utils::find_process;

fn main() {
    env_logger::init();

    const PROCESS_NAME: &str = "Minecraft.Windows";
    const BUFFER: &[u8; 14] = b"Hello, World!\0";
    const BUFFER_SIZE: usize = BUFFER.len();

    let mut bytes_written: usize = 0;

    unsafe {
        info!(
            target: "find_modules",
            "Searching for kernel32.dll and LoadLibrary."
        );

        let h_module = GetModuleHandleA(b"kernel32.dll\0".as_ptr().cast());
        let proc_ll_addr = GetProcAddress(h_module, b"LoadLibrayA\0".as_ptr().cast());

        let proc_pid = find_process(PROCESS_NAME).unwrap();

        info!(
            target: "write_process",
            "Injecting into process PID: {}", proc_pid
        );

        let h_proc = OpenProcess(56, 0, proc_pid);

        let proc_addr = VirtualAllocEx(h_proc, NULL(), BUFFER_SIZE + 1, 12288, 64);

        WriteProcessMemory(
            h_proc,
            proc_addr,
            BUFFER.as_ptr() as *const c_void,
            BUFFER_SIZE,
            &mut bytes_written,
        );

        if bytes_written == 0 {
            error!(
                target: "write_process",
                "Cannot run WriteProcessMemory properly. {}",
                GetLastError()
            );
        }

        info!(
            target: "create_thread",
            "Creating remote thread."
        );

        let thread = CreateRemoteThread(
            h_proc,
            NULL(),
            0,
            Some(std::mem::transmute(proc_ll_addr)),
            proc_addr,
            0,
            bytes_written as *mut usize as *mut u32,
        );

        if thread == NULL() {
            error!(target: "create_thread",
            "Couldn't create remote thread under error: {}", GetLastError());
        }
    }
}
