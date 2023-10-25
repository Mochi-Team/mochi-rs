#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        crate::imports::core::format(core::format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        crate::imports::core::print("");
    }};
    ($($arg:tt)*) => {{
        let string = crate::imports::core::format(core::format_args!($($arg)*));
        crate::import::core::print(&(string));
    }};
}