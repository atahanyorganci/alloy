use std::{mem, ptr::NonNull};

pub use crate::object::{boolean::AlloyBool, float::AlloyFloat, int::AlloyInt};

mod boolean;
mod float;
mod int;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlloyType {
    Int,
    Float,
    Bool,
}

pub type AlloyObjPtr = NonNull<AlloyType>;

pub trait AlloyObj<U>: From<U> {
    fn get(&self) -> U;
    fn set(&mut self, value: U);
    fn get_type() -> AlloyType;
}

pub fn create<Obj, T>(value: T) -> AlloyObjPtr
where
    Obj: AlloyObj<T>,
{
    let obj = Box::from(Obj::from(value));
    let obj_ptr = Box::<Obj>::into_raw(obj);
    unsafe { NonNull::new_unchecked(obj_ptr as *mut AlloyType) }
}

pub unsafe fn destroy(obj_ptr: AlloyObjPtr) {
    match obj_ptr.as_ref() {
        AlloyType::Int => {
            let int_ptr = obj_ptr.as_ptr() as *mut AlloyInt;
            Box::from_raw(int_ptr);
        }
        AlloyType::Float => {
            let float_ptr = obj_ptr.as_ptr() as *mut AlloyFloat;
            Box::from_raw(float_ptr);
        }
        AlloyType::Bool => {
            let bool_ptr = obj_ptr.as_ptr() as *mut AlloyBool;
            Box::from_raw(bool_ptr);
        }
    }
}

pub fn as_float(obj: AlloyObjPtr) -> f64 {
    let ty = unsafe { obj.as_ref() };
    match ty {
        AlloyType::Int => {
            let int: &AlloyInt = unsafe { mem::transmute(ty) };
            int.get() as f64
        }
        AlloyType::Float => {
            let float: &AlloyFloat = unsafe { mem::transmute(ty) };
            float.get()
        }
        AlloyType::Bool => {
            let boolean: &AlloyBool = unsafe { mem::transmute(ty) };
            if boolean.get() {
                1.0
            } else {
                0.0
            }
        }
    }
}

pub fn as_int(obj: AlloyObjPtr) -> i64 {
    let ty = unsafe { obj.as_ref() };
    match ty {
        AlloyType::Int => {
            let int: &AlloyInt = unsafe { mem::transmute(ty) };
            int.get()
        }
        AlloyType::Float => {
            let float: &AlloyFloat = unsafe { mem::transmute(ty) };
            float.get() as i64
        }
        AlloyType::Bool => {
            let boolean: &AlloyBool = unsafe { mem::transmute(ty) };
            if boolean.get() {
                1
            } else {
                0
            }
        }
    }
}

pub fn as_bool(obj: AlloyObjPtr) -> bool {
    let ty = unsafe { obj.as_ref() };
    match ty {
        AlloyType::Int => {
            let int: &AlloyInt = unsafe { mem::transmute(ty) };
            int.get() != 0
        }
        AlloyType::Float => {
            let float: &AlloyFloat = unsafe { mem::transmute(ty) };
            float.get() != 0.0
        }
        AlloyType::Bool => {
            let boolean: &AlloyBool = unsafe { mem::transmute(ty) };
            boolean.get()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::{
        as_bool, as_float, as_int, boolean::AlloyBool, create, destroy, AlloyFloat, AlloyInt,
    };

    #[test]
    fn test_alloy_float_as_f64() {
        let float_ptr = create::<AlloyFloat, f64>(12.0);
        assert_eq!(as_float(float_ptr), 12.0);
        unsafe {
            destroy(float_ptr);
        }
    }

    #[test]
    fn test_alloy_int_as_f64() {
        let intr_ptr = create::<AlloyInt, i64>(12);
        assert_eq!(as_float(intr_ptr), 12.0);
        unsafe {
            destroy(intr_ptr);
        }
    }

    #[test]
    fn test_alloy_bool_as_f64() {
        let bool_ptr = create::<AlloyBool, bool>(true);
        assert_eq!(as_float(bool_ptr), 1.0);
        unsafe {
            destroy(bool_ptr);
        }

        let bool_ptr = create::<AlloyBool, bool>(false);
        assert_eq!(as_float(bool_ptr), 0.0);
        unsafe {
            destroy(bool_ptr);
        }
    }

    #[test]
    fn test_alloy_float_as_i64() {
        let float_ptr = create::<AlloyFloat, f64>(12.0);
        assert_eq!(as_int(float_ptr), 12);
        unsafe {
            destroy(float_ptr);
        }
    }

    #[test]
    fn test_alloy_int_as_i64() {
        let int_ptr = create::<AlloyInt, i64>(12);
        assert_eq!(as_int(int_ptr), 12);
        unsafe {
            destroy(int_ptr);
        }
    }

    #[test]
    fn test_alloy_bool_as_i64() {
        let bool_ptr = create::<AlloyBool, bool>(true);
        assert_eq!(as_int(bool_ptr), 1);
        unsafe {
            destroy(bool_ptr);
        }

        let bool_ptr = create::<AlloyBool, bool>(false);
        assert_eq!(as_int(bool_ptr), 0);
        unsafe {
            destroy(bool_ptr);
        }
    }
    #[test]
    fn test_alloy_float_as_bool() {
        let float_ptr = create::<AlloyFloat, f64>(12.0);
        assert!(as_bool(float_ptr));
        unsafe {
            destroy(float_ptr);
        }

        let float_ptr = create::<AlloyFloat, f64>(0.0);
        assert!(!as_bool(float_ptr));
        unsafe {
            destroy(float_ptr);
        }
    }

    #[test]
    fn test_alloy_int_as_bool() {
        let int_ptr = create::<AlloyInt, i64>(12);
        assert!(as_bool(int_ptr));
        unsafe {
            destroy(int_ptr);
        }
        let int_ptr = create::<AlloyInt, i64>(0);
        assert!(!as_bool(int_ptr));
        unsafe {
            destroy(int_ptr);
        }
    }

    #[test]
    fn test_alloy_bool_as_bool() {
        let bool_ptr = create::<AlloyBool, bool>(true);
        assert!(as_bool(bool_ptr));
        unsafe {
            destroy(bool_ptr);
        }

        let bool_ptr = create::<AlloyBool, bool>(false);
        assert!(!as_bool(bool_ptr));
        unsafe {
            destroy(bool_ptr);
        }
    }
}
