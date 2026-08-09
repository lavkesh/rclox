use crate::value::{Object, Value};
use std::collections::HashMap;

pub struct ClassObject {
    pub name: String,
}

pub struct InstanceObject {
    pub class: *mut Object,
    pub fields: HashMap<String, Value>,
}

impl InstanceObject {
    pub fn class_mut_ref(&mut self) -> &mut ClassObject {
        unsafe { &mut *self.class.as_mut().unwrap().as_class_mut() }
    }
    pub fn class_ref(&self) -> &ClassObject {
        unsafe { &*self.class.as_ref().unwrap().as_class() }
    }
}
