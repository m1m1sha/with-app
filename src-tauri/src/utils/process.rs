use std::{io::Result, mem::size_of_val};
use tokio::process::Command;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
        Threading::{OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE},
    },
};

use super::string::from_utf8_or_gbk;

pub struct Process(HANDLE);

impl Process {
    pub fn open(pid: u32) -> Result<Self> {
        unsafe {
            Ok(Self(OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_TERMINATE,
                false,
                pid,
            )?))
        }
    }

    pub fn kill(self) -> Result<()> {
        unsafe { Ok(TerminateProcess(self.0, 1)?) }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        };
    }
}

pub fn get_process_list(like: String) -> Result<Vec<u32>> {
    let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? };
    let mut pe32 = PROCESSENTRY32::default();
    pe32.dwSize = size_of_val(&pe32) as u32;
    unsafe { Process32First(handle, &mut pe32) }?;
    let mut pids = vec![];
    loop {
        let sz_exe_file = pe32.szExeFile.map(|x| x as u8).to_vec();
        let name = from_utf8_or_gbk(&sz_exe_file);
        if name.contains(&like) {
            pids.push(pe32.th32ProcessID);
        }

        for item in pe32.szExeFile.iter_mut() {
            *item = 0;
        }

        if unsafe { Process32Next(handle, &mut pe32).is_err() } {
            break;
        }
    }

    Ok(pids)
}

pub fn kill_process(name: String) -> Result<()> {
    let pids = get_process_list(name)?;
    for pid in pids {
        if pid == 0 {
            continue;
        }

        tracing::info!("kill: [pid: {}]", pid);
        if let Ok(p) = Process::open(pid) {
            println!("kill");
            let _ = p.kill();
        } else {
            let _ = Command::new("cmd")
                .creation_flags(0x08000000)
                .kill_on_drop(true)
                .args(["/C", "tskill", &format!("{}", pid), "/A"])
                .spawn()
                .unwrap();
        }
    }
    Ok(())
}
