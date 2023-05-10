pub type Result<T> = core::result::Result<T, MochiError>;

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MochiError {
    PtrCast(PtrCastError),
    Node(NodeError),
    JsonParseError,
    Unimplemented
}

impl From<PtrCastError> for MochiError {
    fn from(cast: PtrCastError) -> Self {
        Self::PtrCast(cast)
    }
}

impl From<NodeError> for MochiError {
    fn from(scraping: NodeError) -> Self {
        Self::Node(scraping)
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum NodeError {
    ParserError,
    ModifyError
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PtrCastError {
    NullPointer,
    Utf8NotValid,
    NotArray,
    NotObject,
    NotString,
    NotNumber,
    NotBool,
    NotNode
}

