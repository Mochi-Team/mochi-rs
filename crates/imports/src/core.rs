use alloc::string::String;

use super::html::Node;

use super::error::{Result, MochiError, PtrCastError};

type MutRawBufPtr = *mut u8;
type RawBufPtr = *const u8;
pub(crate) type HostPtr = i32;

#[link(wasm_import_module = "core")]
extern "C" {
    pub(crate) fn copy(ptr: HostPtr) -> HostPtr;
    pub(crate) fn destroy(ptr: HostPtr);

    fn create_array() -> HostPtr;
    fn create_obj() -> HostPtr;
    fn create_string(buf_raw_ptr: RawBufPtr, buf_len: i32) -> HostPtr;
    fn create_bool(value: bool) -> HostPtr;
    fn create_float(value: f64) -> HostPtr;
    fn create_int(value: i64) -> HostPtr;
    fn create_error() -> HostPtr;

    pub(crate) fn ptr_kind(ptr: HostPtr) -> Kind;

    fn string_len(ptr: HostPtr) -> i32;
    fn read_string(ptr: HostPtr, buf_raw_ptr: MutRawBufPtr, buf_len: i32);
    fn read_int(ptr: HostPtr) -> i64;
    fn read_float(ptr: HostPtr) -> f64;
    fn read_bool(ptr: HostPtr) -> bool;

    fn obj_len(ptr: HostPtr) -> usize;
    fn obj_get(ptr: HostPtr, key_raw_ptr: RawBufPtr, len: usize) -> HostPtr;
    fn obj_set(ptr: HostPtr, key_raw_ptr: RawBufPtr, len: usize, value_ptr: HostPtr);
    fn obj_remove(ptr: HostPtr, key_raw_ptr: RawBufPtr, len: usize);
    fn obj_keys(ptr: HostPtr) -> HostPtr;
    fn obj_values(ptr: HostPtr) -> HostPtr;

    fn array_len(ptr: HostPtr) -> i32;
    fn array_get(ptr: HostPtr, idx: i32) -> i32;
    fn array_set(ptr: HostPtr, idx: i32, value_ptr: i32);
    fn array_append(ptr: HostPtr, value_pre: i32);
    fn array_remove(ptr: HostPtr, idx: i32);
}

/// Prints a message to the Aidoku logs.
pub fn print<T: AsRef<str>>(string: T) {
    let string = string.as_ref();
    extern "C" {
        fn print(string: *const u8, size: usize);
    }
    unsafe {
        print(string.as_ptr(), string.len());
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Kind {
    Unknown,
    Null,
    Object,
    Array,
    String,
    Number,
    Bool,
    Node
}

/// References a pointer from the host.
/// 
/// It could be casted to any type, although you should 
/// only cast if you are sure of the type of cast.
///
/// `From<T>` implementations are used to turn a rust value to a host pointer. It allocates 
/// the value in the host.
///
/// `Into<T>` implementations are used to convert a host's value pointer to a rust type.
/// 
#[derive(Debug)]
pub struct PtrRef(HostPtr);

impl PtrRef {
    #[inline]
    pub fn new(ptr: HostPtr) -> Self {
        PtrRef(ptr)
    }

    #[inline]
    pub fn pointer(&self) -> HostPtr {
        self.0
    }

    #[inline]
    pub fn kind(&self) -> Kind {
        unsafe { ptr_kind(self.0) }
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.kind() == Kind::Null
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn as_string(&self) -> Result<String> {
        match self.kind() {
            Kind::String => {
                let str_len = unsafe { string_len(self.0) };
                let mut buf = alloc::vec::Vec::with_capacity(str_len as usize);
                unsafe { 
                    read_string(self.0, buf.as_mut_ptr(), str_len);
                    buf.set_len(str_len as usize);
                }
                String::from_utf8(buf).map_err(|_| MochiError::from(PtrCastError::Utf8NotValid))        
            },
            Kind::Null => {
                Err(MochiError::from(PtrCastError::NullPointer))
            },
            _ => {
                Err(MochiError::from(PtrCastError::NotString))
            }
        }
    }

    pub fn as_object(self) -> Result<ObjectRef> {
        match self.kind() {
            Kind::Object => {
                Ok(ObjectRef(self))
            },
            Kind::Null => {
                Err(MochiError::from(PtrCastError::NullPointer))
            },
            _ => {
                Err(MochiError::from(PtrCastError::NotObject))
            }
        }
    }

    pub fn as_array(self) -> Result<ArrayRef> {
        match self.kind() {
            Kind::Array => {
                Ok(ArrayRef(self, 0, 0))
            },
            Kind::Null => {
                Err(MochiError::from(PtrCastError::NullPointer))
            },
            _ => {
                Err(MochiError::from(PtrCastError::NotArray))
            }
        }
    }

    pub fn as_int(&self) -> Result<i64> {
        match self.kind() {
            Kind::Number | Kind::Bool | Kind::String => {
                Ok(unsafe { read_int(self.0) })    
            },
            Kind::Null => {
                Err(MochiError::from(PtrCastError::NullPointer))
            },
            _ => {
                Err(MochiError::from(PtrCastError::NotNumber))
            }
        }
    }

    pub fn as_float(&self) -> Result<f64> {
        match self.kind() {
            Kind::Number | Kind::Bool | Kind::String => {
                Ok(unsafe { read_float(self.0) })    
            },
            Kind::Null => {
                Err(MochiError::from(PtrCastError::NullPointer))
            },
            _ => {
                Err(MochiError::from(PtrCastError::NotNumber))
            }
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match self.kind() {
            Kind::Number | Kind::Bool => {
                Ok(unsafe { read_bool(self.0) })
            },
            Kind::Null => {
                Err(MochiError::from(PtrCastError::NullPointer))
            },
            _ => {
                Err(MochiError::from(PtrCastError::NotNumber))
            }
        }
    }

    /// Cast the ValueRef to a [Node](crate::html::Node).
    pub fn as_node(&self) -> Result<Node> {
        match self.kind() {
            Kind::Node => {
                Ok(unsafe { Node::from(self.0) })
            },
            Kind::Null => {
                Err(MochiError::from(PtrCastError::NullPointer))
            },
            _ => {
                Err(MochiError::from(PtrCastError::NotArray))
            }
        }
    }
}

impl Clone for PtrRef {
    fn clone(&self) -> Self {
        Self(unsafe { copy(self.0) })
    }
}

impl Drop for PtrRef {
    fn drop(&mut self) {
        unsafe { destroy(self.0) }
    }
}

impl From<i32> for PtrRef {
    fn from(value: i32) -> Self {
        PtrRef(unsafe { create_int(value as i64) })
    }
}

impl From<i64> for PtrRef {
    fn from(value: i64) -> Self {
        PtrRef(unsafe { create_int(value as i64) })
    }
}

impl From<f32> for PtrRef {
    fn from(value: f32) -> Self {
        PtrRef(unsafe { create_float(value as f64) })
    }
}

impl From<f64> for PtrRef {
    fn from(value: f64) -> Self {
        PtrRef(unsafe { create_float(value as f64) })
    }
}

impl From<bool> for PtrRef {
    fn from(value: bool) -> Self {
        PtrRef(unsafe { create_bool(value) })
    }
}

impl From<String> for PtrRef {
    fn from(value: String) -> Self {
        PtrRef( unsafe { create_string(value.as_ptr(), value.len() as i32) })
    }
}

impl From<&str> for PtrRef {
    fn from(value: &str) -> Self {
        PtrRef( unsafe { create_string(value.as_ptr(), value.len() as i32) })
    }
}

impl<T> From<Result<T>> for PtrRef where PtrRef: From<T> {
    fn from(value: Result<T>) -> Self {
        match value {
            Result::Ok(val) => val.into(),
            Result::Err(_) => PtrRef(unsafe { create_error() }),
        }
    }
}

impl Into<String> for PtrRef {
    fn into(self) -> String {
        self.as_string().unwrap_or_default()
    }
}

/// A key-value object, typically shown as a dictionary
/// 
/// 
pub struct ObjectRef(PtrRef);

impl ObjectRef {
    pub fn new() -> Self {
        let ptr: i32 = unsafe { create_obj() };
        Self(PtrRef::new(ptr))
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { obj_len(self.0.0) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: &str) -> PtrRef {
        let ptr = unsafe { obj_get(self.0.0, key.as_ptr(), key.len()) };
        PtrRef::new(ptr)
    }

    #[inline]
    pub fn set(&mut self, key: &str, value: PtrRef) {
        unsafe { obj_set(self.0.0, key.as_ptr(), key.len(), value.0) }
    }

    #[inline]
    pub fn remove(&mut self, key: &str) {
        unsafe { obj_remove(self.0.0, key.as_ptr(), key.len()) }
    }

    pub fn keys(&self) -> ArrayRef {
        let rid = unsafe { obj_keys(self.0 .0) };
        ArrayRef::from(PtrRef::new(rid))
    }

    pub fn values(&self) -> ArrayRef {
        let rid = unsafe { obj_values(self.0 .0) };
        ArrayRef::from(PtrRef::new(rid))
    }
}

impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        Self(PtrRef::new(unsafe { copy(self.0.0) }))
    }
}

impl Default for ObjectRef {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ArrayRef(
    PtrRef,
    i32,
    i32
);

impl ArrayRef {
    pub fn new() -> Self {
        let pid = unsafe { create_array() };
        Self(PtrRef::new(pid), 0, 0)
    }

    #[inline]
    pub fn len(&self) -> i32 {
        unsafe { array_len(self.0 .0) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: i32) -> PtrRef {
        let rid = unsafe { array_get(self.0 .0, index) };
        PtrRef::new(rid)
    }

    #[inline]
    pub fn set(&mut self, index: i32, object: PtrRef) {
        unsafe { array_set(self.0 .0, index, object.0) };
    }

    #[inline]
    pub fn insert(&mut self, value: PtrRef) {
        unsafe { array_append(self.0 .0, value.0) };
        self.2 += 1;
    }

    #[inline]
    pub fn remove(&mut self, index: i32) {
        unsafe { array_remove(self.0 .0, index) };
        self.2 -= 1;
    }

    #[inline]
    pub fn ptr(&self) -> i32 {
        self.0.pointer()
    }
}

impl Iterator for ArrayRef {
    type Item = PtrRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 > self.2 || self.2 == i32::MAX {
            return None;
        }
        let value_ref = self.get(self.1);
        self.1 += 1;
        Some(value_ref)
    }
}

impl DoubleEndedIterator for ArrayRef {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.1 > self.2 || self.2 == i32::MAX {
            return None;
        }
        let value_ref = self.get(self.2);
        self.2 = self.2.wrapping_sub(1);
        Some(value_ref)
    }
}

impl FromIterator<PtrRef> for ArrayRef {
    fn from_iter<T: IntoIterator<Item = PtrRef>>(iter: T) -> Self {
        let mut array = Self::new();
        for value in iter {
            array.insert(value);
        }
        array
    }
}

impl From<PtrRef> for ArrayRef {
    fn from(ptrref: PtrRef) -> Self {
        let length = unsafe { array_len(ptrref.0) };
        Self(ptrref, 0, length.wrapping_sub(1))
    }
}

impl Clone for ArrayRef {
    fn clone(&self) -> Self {
        let ptr = unsafe { copy(self.0.0) };
        Self(PtrRef::new(ptr), self.1, self.2)
    }
}

impl Default for ArrayRef {
    fn default() -> Self {
        Self::new()
    }
}