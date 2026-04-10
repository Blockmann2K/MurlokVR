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
    // Raw Windows Handle To File Mapping Object
    handle: HANDLE,

    // Starting Address of the Mapped File Mapping Object.
    // Stored Solely for Clean Up - 'UnmapViewOfFile' Requires This Address in 'Drop'.
    // 'Option' Because the Mapping May Not Yet Exist or May Have Already Been Released.
    memory_address: Option<MEMORY_MAPPED_VIEW_ADDRESS>,

    // Zero-Sized Marker That Ties This Struct to 'T' Without Storing a 'T'.
    // Also Communicates to the Drop Checker That 'SharedMemory<T>' Logically Owns a 'T'.
    // 'PhantomData' Because 'T' Appears in the Trait Bound but Not in Any Real Field.
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

            return None;
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

        if handle.is_null() {
            log_last_error("Create");

            return None;
        }

        log_success("Created");

        Some(Self {
            handle,
            memory_address: None,
            marker: PhantomData,
        })
    }

    pub fn map_view_as_mut(&mut self) -> Option<&mut T> {
        let memory_address = unsafe {
            MapViewOfFile(
                self.handle,         // File Mapping Object
                FILE_MAP_ALL_ACCESS, // Desired Access Permissions
                0,                   // Upper 32 Bits View Offset
                0,                   // Lower 32 Bits View Offset
                0,                   // Amount of Bytes To View | 0 => To the End of the File Mapping Object.
            )
        };

        if memory_address.Value.is_null() {
            log_last_error("Map");

            return None;
        }

        // Store for Clean Up
        self.memory_address = Some(memory_address);

        log_success("Mapped");

        let memory_address = memory_address.Value as *mut T;

        // Dereference Raw Pointer and Return a Mutable Reference to 'T'.
        Some(unsafe { &mut *memory_address })
    }
}

impl<T: SharedMemorySafe + Debug> Drop for SharedMemory<T> {
    fn drop(&mut self) {
        let memory_address = match self.memory_address {
            Some(n) => {
                println!("DEBUG: Valid Memory Address for Clean Up!");
                n
            }

            None => {
                println!("DEBUG: Invalid Memory Address for Clean Up!");
                return;
            }
        };

        println!("DEBUG: Clean Up File Mapping Object: {:?}!", self.handle);

        unsafe {
            UnmapViewOfFile(memory_address);

            CloseHandle(self.handle);
        }

        log_success("Cleaned Up");
    }
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
