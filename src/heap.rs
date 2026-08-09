use crate::value::{Object, ObjectType, Value};
use std::collections::HashMap;
use std::ptr::null_mut;

pub struct Heap {
    objects: *mut Object,
    gray_stack: Vec<*mut Object>,
    pub(crate) interned_strings: HashMap<String, *mut Object>,
}
impl Heap {
    pub fn new() -> Self {
        Self {
            objects: null_mut(),
            gray_stack: Vec::new(),
            interned_strings: HashMap::new(),
        }
    }
    pub fn allocate(&mut self, obj_type: ObjectType) -> *mut Object {
        let obj = Box::new(Object {
            obj_type,
            is_marked: false,
            next: self.objects,
        });
        let ptr = Box::into_raw(obj);
        self.objects = ptr;
        ptr
    }

    pub fn mark_object(&mut self, obj: *mut Object) {
        unsafe {
            if !obj.is_null() && !obj.as_ref().unwrap().is_marked {
                obj.as_mut().unwrap().is_marked = true;
                self.gray_stack.push(obj);
            }
        }
    }
    pub fn trace_references(&mut self) {
        while !self.gray_stack.is_empty() {
            let ob = unsafe { self.gray_stack.pop().unwrap().as_mut().unwrap() };
            match &ob.obj_type {
                ObjectType::Native(_) | ObjectType::String(_) => {}
                ObjectType::UpValue(upvalue) => self.mark_value(upvalue.closed.clone()),
                ObjectType::Function(function) => self.mark_array(&function.chunk.constants),
                ObjectType::Closure(closure) => {
                    self.mark_object(closure.function);
                    closure.upvalues.iter().for_each(|up| self.mark_object(*up));
                }
                ObjectType::Array(array) => self.mark_array(array),
                ObjectType::Class(_) => {}
            }
        }
    }
    pub fn remove_strings(&mut self) {
        unsafe {
            self.interned_strings.retain(|_key, value| value.as_ref().unwrap().is_marked);
        }
    }
    pub fn sweep(&mut self) {
        let mut current = self.objects;
        let mut previous: *mut Object = null_mut();
        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                let is_marked = (*current).is_marked;
                if is_marked {
                    (*current).is_marked = false;
                    previous = current;
                } else {
                    if !previous.is_null() {
                        (*previous).next = next;
                    } else {
                        self.objects = next;
                    }
                    self.free(current);
                }
                current = next;
            }
        }
    }
    pub fn mark_array(&mut self, constants: &Vec<Value>) {
        constants.iter().for_each(|constant| self.mark_value(constant.clone()));
    }
    pub fn mark_value(&mut self, val: Value) {
        if val.is_object() {
            self.mark_object(val.as_object());
        }
    }
    pub fn free(&mut self, obj: *mut Object) {
        unsafe {
            drop(Box::from_raw(obj));
        }
    }
}
impl Drop for Heap {
    fn drop(&mut self) {
        let mut current = self.objects;
        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                drop(Box::from_raw(current));
                current = next;
            }
        }
        self.objects = null_mut();
    }
}
