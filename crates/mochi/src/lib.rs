#![doc = include_str!("../../../README.md")]
#![no_std]
#![feature(
    core_intrinsics,
    alloc_error_handler,
    fmt_internals,
    panic_info_message
)]

#[cfg_attr(feature = "dlmalloc", global_allocator)]
static ALLOCATOR: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

fn as_abort<T: AsRef<str>>(message: T, file: T, line: u32, column: u32) -> ! {
    extern "C" {
        #[link_name = "abort"]
        fn _abort(message: *const u8, file: *const u8, line: i32, column: i32);
    }
    extern crate alloc;
    use alloc::alloc::{alloc_zeroed, dealloc};
    use core::{alloc::Layout, ptr::copy};

    let message = message.as_ref();
    let file = file.as_ref();

    // Basically, AssemblyScript places 4 bytes before the string slice to denote
    // its length. This is why we need the extra 8 bytes.
    if let Ok(layout) =
        Layout::from_size_align(8 + message.len() + file.len(), core::mem::align_of::<u8>())
    {
        unsafe {
            let message_len_ptr = alloc_zeroed(layout) as *mut i32;
            *message_len_ptr = i32::try_from(message.len()).unwrap_or(-1);

            let message_ptr = message_len_ptr.add(1) as *mut u8;
            copy::<u8>(message.as_ptr(), message_ptr, message.len());

            let file_len_ptr = message_len_ptr.add(message.len()) as *mut i32;
            *file_len_ptr = i32::try_from(file.len()).unwrap_or(-1);

            let file_ptr = file_len_ptr.add(1) as *mut u8;
            copy::<u8>(file.as_ptr(), file_ptr, file.len());

            _abort(
                message_ptr,
                file_ptr,
                line.try_into().unwrap_or(-1),
                column.try_into().unwrap_or(-1),
            );

            dealloc(message_len_ptr as *mut u8, layout);
            dealloc(message_ptr, layout);
            dealloc(file_len_ptr as *mut u8, layout);
            dealloc(file_ptr, layout);
        }
    }

    core::intrinsics::abort()
}

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handle(info: &core::panic::PanicInfo) -> ! {
    // use crate::imports::Write;
    extern crate alloc;
    use alloc::string::String;
    use ::core::fmt::Write;

    let (file, line, col) = if let Some(location) = info.location() {
        (location.file(), location.line(), location.column())
    } else {
        ("", 0, 0)
    };

    let message = if let Some(args) = info.message() {
        let mut string = String::with_capacity(args.estimated_capacity());
        string.write_fmt(*args).unwrap_or_default();
        string
    } else {
        String::new()
    };

    as_abort(message, String::from(file), line, col)
}

mod macros;
pub use crate::macros::*;

mod imports;
pub use imports::error;

pub mod std {
    pub use crate::imports::*;
}

pub use mochi_bind::*;

pub mod structs;

#[cfg(feature = "extractors")]
pub mod extractors;