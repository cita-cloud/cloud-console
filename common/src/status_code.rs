use serde_repr::Serialize_repr;

#[derive(Serialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum StatusCode {
    Error = 0,
    Success = 1,
}
