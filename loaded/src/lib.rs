use std::mem::size_of;
use widestring::U16CStr;
use windows::{
    core::Result,
    Win32::{
        Foundation::CloseHandle,
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W,
                TH32CS_SNAPMODULE,
            },
            Threading::GetCurrentProcessId,
        },
    },
};

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub base: usize,
}

pub fn get_loaded_modules() -> Result<Vec<ModuleInfo>> {
    let mut modules = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, GetCurrentProcessId())?;
        let mut entry = MODULEENTRY32W::default();
        entry.dwSize = size_of::<MODULEENTRY32W>() as u32;

        if Module32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                modules.push(ModuleInfo {
                    name: U16CStr::from_slice_truncate(&entry.szExePath)
                    .unwrap()
                    .to_string_lossy(),
                             base: entry.modBaseAddr as usize,
                });
                if Module32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
        CloseHandle(snapshot)?;
    }
    Ok(modules)
}
