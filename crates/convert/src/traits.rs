pub trait FromWasm {
    type Value: WasmValue;
    fn from_wasm(val: Self::Value) -> Self;
}

pub trait ToWasm {
    type Value: WasmValue;
    fn to_wasm(self) -> Self::Value;
}

pub trait RefFromWasm {
    type Value: WasmValue;
    type Anchor: std::ops::Deref<Target = Self>;
    fn ref_from_wasm(val: Self::Value) -> Self::Anchor;
}

pub unsafe trait WasmValue {}

macro_rules! wasm_values {
    ($($t:ident)*) => { $(unsafe impl WasmValue for $t {})* };
}

wasm_values! { u32 i32 f32 u64 i64 f64 }