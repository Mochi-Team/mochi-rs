extern crate alloc;
use alloc::string::String;

// Useful for only trying to return a value
// without dropping the string if it's not None.
pub fn optional_str_ptr(value: Option<String>) -> (i32, i32) {
    match value {
        Some(string) => {
            let str_ptr = string.as_ptr();
            let str_len = string.len();
            // Need to forget this string since it gets dropped
            // since it get's out of scope.
            core::mem::forget(string);
            (str_ptr as i32, str_len as i32)
        },
        _ => (-1, -1),
    }
}