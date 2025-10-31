use std::ffi::CStr;
use windows::{
    core::{Result, PCSTR},
    Win32::{
        System::{
            Diagnostics::Debug::{IMAGE_DIRECTORY_ENTRY_EXPORT, IMAGE_NT_HEADERS64},
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_EXPORT_DIRECTORY},
        },
    },
};

#[derive(Debug)]
pub struct HookedFunctionInfo {
    pub name: String,
    pub target_address: usize,
}

pub fn find_hooks_in_dll(dll_name: &str) -> Result<Vec<HookedFunctionInfo>> {
    let mut hooks = Vec::new();
    unsafe {
        let dll_name_nul = format!("{}\0", dll_name);
        let dll_handle = match GetModuleHandleA(PCSTR(dll_name_nul.as_ptr())) {
            Ok(handle) if !handle.is_invalid() => handle,
            _ => return Ok(hooks), // Return empty list if DLL not found
        };
        let base_addr = dll_handle.0 as usize;
        let dos_header = base_addr as *const IMAGE_DOS_HEADER;

        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            return Ok(hooks);
        }

        let nt_headers = (base_addr + (*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
        let export_dir_rva = (*nt_headers).OptionalHeader.DataDirectory
        [IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize]
        .VirtualAddress;

        if export_dir_rva == 0 {
            return Ok(hooks);
        }

        let export_dir = (base_addr + export_dir_rva as usize) as *const IMAGE_EXPORT_DIRECTORY;
        let names_rva = (base_addr + (*export_dir).AddressOfNames as usize) as *const u32;

        for i in 0..(*export_dir).NumberOfNames {
            let name_rva = *names_rva.add(i as usize);
            let name_ptr = (base_addr + name_rva as usize) as *const u8;
            let name = CStr::from_ptr(name_ptr as *const i8).to_str().unwrap_or_default();

            if let Some(proc_addr) = GetProcAddress(dll_handle, PCSTR(name_ptr)) {
                let addr = proc_addr as usize;
                // Check for a JMP instruction (opcode 0xE9)
                if *(addr as *const u8) == 0xE9 {
                    let offset = *(addr.wrapping_add(1) as *const i32);
                    let target_address = (addr.wrapping_add(5)).wrapping_add(offset as usize);
                    hooks.push(HookedFunctionInfo {
                        name: name.to_string(),
                               target_address,
                    });
                }
            }
        }
    }
    Ok(hooks)
}
