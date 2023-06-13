#![no_std]

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        mochi_imports::core::format(core::format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        mochi_imports::core::print("");
    }};
    ($($arg:tt)*) => {{
        let string = mochi_imports::core::format(core::format_args!($($arg)*));
        mochi_imports::core::print(&(string));
    }};
}