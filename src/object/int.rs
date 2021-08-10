use super::{AlloyObj, AlloyType};

#[repr(C)]
pub struct AlloyInt {
    ty: AlloyType,
    value: i64,
}

impl Default for AlloyInt {
    fn default() -> Self {
        AlloyInt {
            ty: AlloyType::Int,
            value: 0,
        }
    }
}

impl AlloyInt {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<i64> for AlloyInt {
    fn from(value: i64) -> Self {
        AlloyInt {
            ty: AlloyType::Int,
            value,
        }
    }
}

impl AlloyObj<i64> for AlloyInt {
    fn get_type() -> AlloyType {
        AlloyType::Int
    }

    fn get(&self) -> i64 {
        self.value
    }

    fn set(&mut self, value: i64) {
        self.value = value;
    }
}
