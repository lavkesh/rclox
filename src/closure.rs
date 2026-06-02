use crate::function::FunctionObject;
use crate::value::{Object, Value};
use std::ptr::null_mut;

pub struct ClosureObject {
    pub function: *mut Object,
    pub upvalues: Vec<*mut Object>,
}
impl ClosureObject {
    pub fn new(function: *mut Object) -> Self {
        unsafe {
            let upvalue_count = function.as_mut().unwrap().as_function_mut().upvalue_count;
            Self {
                function,
                upvalues: vec![null_mut(); upvalue_count],
            }
        }
    }
    pub unsafe fn function_ref(&self) -> &FunctionObject {
        unsafe { &*self.function.as_ref().unwrap().as_function() }
    }
    pub unsafe fn function_mut(&mut self) -> &mut FunctionObject {
        unsafe { &mut *self.function.as_mut().unwrap().as_function_mut() }
    }
}
pub struct CallFrame {
    pub closure: *mut Object,
    pub ip: usize,
    pub stack_base: usize,
}
#[derive(Debug, Clone)]
pub struct CompilerUpvalue {
    pub index: u8,
    pub is_local: bool,
}

pub struct UpValueObject {
    pub location: *mut Value, // pointer into VM.stack
    pub closed: Value,
}
impl UpValueObject {
    pub fn new(location: *mut Value) -> Self {
        Self { location, closed: Value::Nil }
    }
}
