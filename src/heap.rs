use crate::value::{Object, ObjectType, Value};

pub struct Heap {
    objects: *mut Object,
    pub(crate) total: usize,
}
impl Heap {
    pub fn new() -> Self {
        Self {
            objects: std::ptr::null_mut(),
            total: 0,
        }
    }
    pub fn allocate(&mut self, obj_type: ObjectType) -> *mut Object {
        let obj = Box::new(Object {
            obj_type,
            is_marked: false,
            next: self.objects,
        });
        self.total += 1;
        let ptr = Box::into_raw(obj);
        self.objects = ptr;
        ptr
    }

    pub fn mark_object(&mut self, obj: *mut Object) {
        unsafe {
            if !obj.is_null() {
                obj.as_mut().unwrap().is_marked = true;
            }
        }
    }
    pub fn mark_value(&mut self, val: Value) {
        if val.is_object() {
            self.mark_object(val.as_object());
        }
    }
    pub fn free(&mut self, obj: *mut Object) {
        self.total -= 1;
    }
    pub fn collect_garbage(&mut self) {}
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
        self.objects = std::ptr::null_mut();
    }
}
