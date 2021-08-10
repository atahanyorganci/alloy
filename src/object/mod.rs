use std::{mem, ptr::NonNull};

pub use crate::object::{float::AlloyFloat, int::AlloyInt};

mod float;
mod int;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlloyType {
    Int,
    Float,
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
    let ty = obj_ptr.as_ref();
    match ty {
        AlloyType::Int => {
            let int_ptr = obj_ptr.as_ptr() as *mut AlloyInt;
            Box::from_raw(int_ptr);
        }
        AlloyType::Float => {
            let float_ptr = obj_ptr.as_ptr() as *mut AlloyFloat;
            Box::from_raw(float_ptr);
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
    }
}

#[cfg(test)]
mod tests {
    use crate::object::{as_float, as_int, create, destroy, AlloyFloat, AlloyInt};

    #[test]
    fn test_float_struct_inheritance_for_f64() {
        let float_ptr = create::<AlloyFloat, f64>(12.0);
        assert_eq!(as_float(float_ptr), 12.0);
        unsafe {
            destroy(float_ptr);
        }
    }

    #[test]
    fn test_int_struct_inheritance_for_f64() {
        let float_ptr = create::<AlloyInt, i64>(12);
        assert_eq!(as_float(float_ptr), 12.0);
        unsafe {
            destroy(float_ptr);
        }
    }

    #[test]
    fn test_float_struct_inheritance_for_i64() {
        let float_ptr = create::<AlloyFloat, f64>(12.0);
        assert_eq!(as_int(float_ptr), 12);
        unsafe {
            destroy(float_ptr);
        }
    }

    #[test]
    fn test_int_struct_inheritance_for_i64() {
        let float_ptr = create::<AlloyInt, i64>(12);
        assert_eq!(as_int(float_ptr), 12);
        unsafe {
            destroy(float_ptr);
        }
    }
}
