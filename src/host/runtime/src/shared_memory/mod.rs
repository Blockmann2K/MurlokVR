// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
#![allow(unused_imports)] // TODO: Remove This When We Actually Use All the Imports.

// The Rust Standard Library
use std::{fmt::Debug, marker::PhantomData, mem::size_of, ptr::null};

// Raw Windows API Bindings
// Docs: https://docs.rs/windows-sys/0.61.2/windows_sys/
use windows_sys::{Win32::Foundation::*, Win32::Security::*, Win32::System::Memory::*, core::*};

//-----------------------------------------------------------------------------
// Shared Memory Struct (Windows File Mapping Object)
//-----------------------------------------------------------------------------
pub struct SharedMemory<T: SharedMemorySafe + Debug> {
    // Raw Windows Handle to File Mapping Object
    handle: HANDLE,

    // Starting Address of the Mapped File Mapping Object
    memory_address: Option<MEMORY_MAPPED_VIEW_ADDRESS>,

    // Zero-Sized Marker That Ties This Struct to 'T' Without Storing a 'T'.
    // Required Because 'T' Appears in the Trait Bound but Not in Any Real Field.
    marker: PhantomData<T>,
}

//-----------------------------------------------------------------------------
// Shared Memory Marker Trait
//-----------------------------------------------------------------------------
pub trait SharedMemorySafe {} // Implement This Trait for Your Data Type To Enable Shared Memory Support.

//-----------------------------------------------------------------------------
// Shared Memory – Implementations
//-----------------------------------------------------------------------------
impl<T: SharedMemorySafe + Debug> SharedMemory<T> {
    pub fn create() -> Option<Self> {
        let region_size = size_of::<T>() as u32; // Size in Bytes of 'T', Cast to 'u32' for FFI Compatibility.

        println!("DEBUG: Shared Memory Region Size: {} Bytes", region_size);

        if region_size == 0 {
            println!("DEBUG: Failed To Define Shared Memory Region Size!");
        }

        let handle = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,                 // "Anonymous" File Mapping Object - Lives Only in RAM.
                null(),                               // Default Security Attributes
                PAGE_READWRITE,                       // Memory is Readable and Writable
                0,                                    // Upper 32 Bits
                region_size,                          // Lower 32 Bits
                w!("Global\\MurlokVR_Shared_Memory"), // Named File Mapping Object - Other Processes Can Open It With This Name.
            )
        };

        match handle.is_null() {
            true => {
                log_last_error("Create");
                None
            }

            false => {
                log_success("Created");
                Some(Self {
                    handle,
                    memory_address: None,
                    marker: PhantomData,
                })
            }
        }
    }

    pub fn map_with_all_access(&mut self) {
        let file_mapping_object = self.handle;

        let memory_address = unsafe {
            MapViewOfFile(
                file_mapping_object, // Windows File Mapping Object
                FILE_MAP_ALL_ACCESS, // Desired Access Permissions
                0,                   // Upper 32 Bits View Offset
                0,                   // Lower 32 Bits View Offset
                0,                   // Amount of Bytes To View | 0 => To the End of the File Mapping Object
            )
        };

        match memory_address.Value.is_null() {
            true => {
                log_last_error("Map");

                self.memory_address = None;
            }

            false => {
                log_success("Mapped");

                self.memory_address = Some(memory_address);
            }
        }
    }

    pub fn read(&self) {
        validate_memory_address(self.memory_address, "Read");

        let memory_address = match self.memory_address {
            Some(n) => n,
            None => return,
        };

        let memory_address = memory_address.Value as *mut T;

        println!("DEBUG: Reading...");
        println!("DEBUG: Content: {:?}", unsafe { memory_address.read() });
    }

    pub fn write(&self, content: T) {
        validate_memory_address(self.memory_address, "Write");

        let memory_address = match self.memory_address {
            Some(n) => n,
            None => return,
        };

        let memory_address = memory_address.Value as *mut T;

        println!("DEBUG: Writing...");
        unsafe { memory_address.write(content) };
    }
}

impl<T: SharedMemorySafe + Debug> Drop for SharedMemory<T> {
    fn drop(&mut self) {
        validate_memory_address(self.memory_address, "Clean Up");

        let memory_address = match self.memory_address {
            Some(n) => n,
            None => return,
        };

        println!("DEBUG: Clean Up File Mapping Object: {:?}!", self.handle);

        let file_mapping_object = self.handle;

        unsafe {
            UnmapViewOfFile(memory_address);

            CloseHandle(file_mapping_object);
        }

        log_success("Cleaned Up");
    }
}

//-----------------------------------------------------------------------------
// Validate Memory Address Function
//-----------------------------------------------------------------------------
fn validate_memory_address(memory_address: Option<MEMORY_MAPPED_VIEW_ADDRESS>, operation: &str) -> Option<MEMORY_MAPPED_VIEW_ADDRESS> {
    match memory_address {
        Some(_) => {
            println!("DEBUG: Memory Address Is Valid To {}!", operation);
        }

        None => {
            println!("DEBUG: Memory Address Is Invalid To {}!", operation);
        }
    }

    memory_address
}

//-----------------------------------------------------------------------------
// Logging Functions
//-----------------------------------------------------------------------------
fn log_last_error(operation: &str) {
    let error_code = unsafe { GetLastError() };

    match error_code {
        5 => println!("DEBUG: Access Denied! Run Again With Elevated Privileges! - Error Code: {}", error_code),
        _ => println!("DEBUG: Failed To {} Shared Memory Region! - Error Code: {}", operation, error_code),
    }
}

fn log_success(operation: &str) {
    println!("DEBUG: Successfully {} Shared Memory Region!", operation);
}
