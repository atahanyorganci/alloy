use std::fmt;

use super::{AlloyObj, AlloyType};

#[repr(C)]
pub struct AlloyFloat {
    ty: AlloyType,
    value: f64,
}

impl Default for AlloyFloat {
    fn default() -> Self {
        AlloyFloat {
            ty: AlloyType::Float,
            value: 0.0,
        }
    }
}

impl AlloyFloat {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<f64> for AlloyFloat {
    fn from(value: f64) -> Self {
        AlloyFloat {
            ty: AlloyType::Float,
            value,
        }
    }
}

impl AlloyObj<f64> for AlloyFloat {
    fn get(&self) -> f64 {
        self.value
    }

    fn get_type() -> AlloyType {
        AlloyType::Float
    }

    fn set(&mut self, value: f64) {
        self.value = value;
    }
}

impl fmt::Debug for AlloyFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Float({})", self.value)
    }
}
