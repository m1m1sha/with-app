use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Threading::{
        OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE,
    },
};

pub struct Process(HANDLE);

impl Process {
    pub fn open(pid: u32) -> std::io::Result<Self> {
        unsafe {
            Ok(Self(OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_TERMINATE,
                false,
                pid,
            )?))
        }
    }

    pub fn kill(self) -> std::io::Result<()> {
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
