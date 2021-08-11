use std::fmt;

use super::{AlloyObj, AlloyType};

#[repr(C)]
pub struct AlloyBool {
    ty: AlloyType,
    value: bool,
}

impl AlloyObj<bool> for AlloyBool {
    fn get(&self) -> bool {
        self.value
    }

    fn set(&mut self, value: bool) {
        self.value = value;
    }

    fn get_type() -> AlloyType {
        AlloyType::Bool
    }
}

impl From<bool> for AlloyBool {
    fn from(value: bool) -> Self {
        AlloyBool {
            ty: AlloyType::Bool,
            value,
        }
    }
}

impl fmt::Debug for AlloyBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value {
            write!(f, "AlloyTrue")
        } else {
            write!(f, "AlloyFalse")
        }
    }
}
