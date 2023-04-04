// Implementation was inspired by wasm-bindgen
//
// https://github.com/rustwasm/wasm-bindgen/blob/b41de3f22fc2427babe3a7d4184c369651452136/src/convert/slices.rs

pub use crate::traits::{WasmValue, FromWasm, ToWasm, RefFromWasm};

unsafe impl WasmValue for () {}
unsafe impl WasmValue for WasmSlice {}

#[repr(C)]
pub struct WasmSlice {
    pub ptr: u32,
    pub len: u32,
}

impl<T> FromWasm for Vec<T> where Box<[T]>: FromWasm<Value = WasmSlice> {
    type Value = <Box<[T]> as FromWasm>::Value;

    fn from_wasm(val: Self::Value) -> Self {
        <Box<[T]>>::from_wasm(val).into()
    }
} 

impl<T> ToWasm for Vec<T> where Box<[T]>: ToWasm<Value = WasmSlice> {
    type Value = <Box<[T]> as ToWasm>::Value;

    fn to_wasm(self) -> Self::Value {
        self.into_boxed_slice().to_wasm()
    }
} 

impl FromWasm for String {
    type Value = <Vec<u8> as FromWasm>::Value;

    fn from_wasm(value: Self::Value) -> Self {
        unsafe { String::from_utf8_unchecked(<Vec<u8>>::from_wasm(value)) }
    }
}

impl ToWasm for String {
    type Value = <Vec<u8> as ToWasm>::Value;

    fn to_wasm(self) -> Self::Value {
        let ptr = self.as_ptr();
        let len = self.len();
        std::mem::forget(self);
        WasmSlice {
            ptr: ptr as u32,
            len: len as u32,
        }
    }
}

macro_rules! vectors {
    ($($t:ident)*) => {
        $(
            impl FromWasm for Box<[$t]> {
                type Value = WasmSlice;

                #[inline]
                fn from_wasm(val: Self::Value) -> Self {
                    let ptr = <*mut $t>::from_wasm(val.ptr);
                    let len = val.len as usize;
                    unsafe { Vec::from_raw_parts(ptr, len, len).into_boxed_slice() }
                }
            }

            impl ToWasm for Box<[$t]> {
                type Value = WasmSlice;

                #[inline]
                fn to_wasm(self) -> Self::Value {
                    let ptr = self.as_ptr();
                    let len = self.len();
                    std::mem::forget(self);
                    WasmSlice {
                        ptr: ptr as u32,
                        len: len as u32,
                    }
                }
            }

            impl RefFromWasm for [$t] {
                type Value = WasmSlice;
                type Anchor = Box<[$t]>;

                #[inline]
                fn ref_from_wasm(val: Self::Value) -> Self::Anchor {
                    <Box<[$t]>>::from_wasm(val)
                }
            }
        )*
    };
}

vectors! {
    u8 i8 u16 i16 u32 i32 usize isize f32 f64
}

macro_rules! type_wasm_native {
    ($($t:tt as $c:tt)*) => ($(
        impl FromWasm for $t {
            type Value = $c;

            #[inline]
            fn from_wasm(val: $c) -> Self { val as $t }
        }

        impl ToWasm for $t {
            type Value = $c;

            #[inline]
            fn to_wasm(self) -> Self::Value { self as Self::Value }
        }
    )*)
}

type_wasm_native!(
    i32 as i32
    isize as i32
    u32 as u32
    usize as u32
    i64 as i64
    u64 as u64
    f32 as f32
    f64 as f64
);

impl<T> FromWasm for *mut T {
    type Value = u32;

    #[inline]
    fn from_wasm(val: u32) -> *mut T {
        val as *mut T
    }
}

impl<T> ToWasm for *mut T {
    type Value = u32;

    fn to_wasm(self) -> Self::Value {
        self as Self::Value
    }
}

impl RefFromWasm for str {
    type Value = <[u8] as RefFromWasm>::Value;
    type Anchor = Box<str>;

    #[inline]
    fn ref_from_wasm(val: Self::Value) -> Self::Anchor {
        unsafe { std::mem::transmute::<Box<[u8]>, Box<str>>(<Box<[u8]>>::from_wasm(val)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let test = String::from("Test");
        let slice = WasmSlice {
            ptr: test.as_ptr() as u32,
            len: (test.len() + 1) as u32,
        };
        let hm = <String as FromWasm>::from_wasm(slice);
        assert_eq!(test, hm);
    }
}
