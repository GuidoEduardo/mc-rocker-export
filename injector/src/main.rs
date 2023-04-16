use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};

use winapi::shared::minwindef::FALSE;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

fn main() {
    println!("Finding process");

    let system = System::new_all();

    let processes_map = system.processes_by_name("Minecraft.Windows");
    let processes: Vec<&Process> = processes_map.into_iter().collect();

    let desired_access = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;

    let process_handle = unsafe {
        OpenProcess(
            desired_access,
            FALSE,
            processes.first().unwrap().pid().as_u32(),
        )
    };

    println!("Processo aberto com sucesso.");

    println!("{:?}", process_handle);

    unsafe {
        winapi::um::handleapi::CloseHandle(process_handle);
    }
}
