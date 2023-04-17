use log::{error, info};
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};

pub fn find_process(process_name: &str) -> Result<u32, &'static str> {
    info!(target: "find_process", "Searching process with name: {}", process_name);

    let system = System::new_all();

    let process_map = system.processes_by_name(process_name);

    let processes: Vec<&Process> = process_map.into_iter().collect();

    if processes.is_empty() {
        error!(
            target: "find_process",
            "Process {} not found.", process_name
        );

        return Err("Process not found.");
    }

    Ok(processes.first().unwrap().pid().as_u32())
}
