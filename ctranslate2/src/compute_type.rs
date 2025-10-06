/// Compute type (example) — adjust to match your C API
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComputeType {
    Default = 0,
    Float16 = 1,
    Int8 = 2,
}
