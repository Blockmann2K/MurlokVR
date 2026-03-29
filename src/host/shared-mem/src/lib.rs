// Windows API Bindings for Memory Management and Foundation Types
// Docs: https://docs.rs/windows-sys/0.61.2/windows_sys/
#![allow(unused_imports)] // TODO: Remove This When We Actually Use the Imports.
use windows_sys::{Win32::Foundation::*, Win32::Security::*, Win32::System::Memory::*, core::*};

pub struct SharedMemory {
    handle: HANDLE,
}

impl SharedMemory {
    pub fn new(size: u32) -> Result<Self, WIN32_ERROR> {
        let handle = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                std::ptr::null(),
                PAGE_READWRITE,
                0,
                size,
                w!("MurlokVR_Shared_Mem"),
            )
        };

        match handle.is_null() {
            true => {
                let error = unsafe { GetLastError() };
                println!("Failed To Create Shared Memory: {}", error);
                Err(error)
            }

            false => {
                println!("Shared Memory Created Successfully!");
                Ok(SharedMemory { handle })
            }
        }
    }
}
