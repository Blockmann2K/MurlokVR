// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
#![allow(unused_imports)] // TODO: Remove This When We Actually Use All the Imports.

// The Rust Standard Library
use std::{marker::PhantomData, mem::size_of, ptr::null};

// Raw Windows API Bindings
// Docs: https://docs.rs/windows-sys/0.61.2/windows_sys/
use windows_sys::{Win32::Foundation::*, Win32::Security::*, Win32::System::Memory::*, core::*};

//-----------------------------------------------------------------------------
// Shared Memory Struct (Windows File Mapping Object)
//-----------------------------------------------------------------------------
pub struct SharedMemory<T: SharedMemorySafe> {
    // Raw Windows Handle to File Mapping Object
    handle: HANDLE,

    // Zero-Sized Marker That Ties This Struct to 'T' Without Storing a 'T'.
    // Required Because 'T' Appears in the Trait Bound but Not in Any Real Field.
    marker: PhantomData<T>,
}

//-----------------------------------------------------------------------------
// Shared Memory Marker Trait
//-----------------------------------------------------------------------------
pub trait SharedMemorySafe {} // Implement This Trait for Your Data Type To Enable Shared Memory Support.

//-----------------------------------------------------------------------------
// Shared Memory – Implementation
//-----------------------------------------------------------------------------
impl<T: SharedMemorySafe> SharedMemory<T> {
    pub fn create() -> Option<Self> {
        let region_size = size_of::<T>() as u32; // Size in Bytes of 'T', Cast to 'u32' for FFI Compatibility.

        println!("DEBUG: Shared Memory Region Size: {} Bits", region_size);

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
                Some(Self { handle, marker: PhantomData })
            }
        }
    }

    pub fn map_with_all_access(&self) -> Option<MEMORY_MAPPED_VIEW_ADDRESS> {
        let file_mapping_object = self.handle;

        let region_start_address = unsafe {
            MapViewOfFile(
                file_mapping_object, // ...
                FILE_MAP_ALL_ACCESS, // ...
                0,                   // ...
                0,                   // ...
                0,                   // ...
            )
        };

        match region_start_address.Value.is_null() {
            true => {
                log_last_error("Map");
                None
            }

            false => {
                log_success("Mapped");
                Some(region_start_address)
            }
        }
    }

    // pub fn read
    // pub fn write
    // impl Drop for SharedMemory
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
