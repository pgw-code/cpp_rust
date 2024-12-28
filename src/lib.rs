use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[repr(C)]
pub struct Node {
    data: i32,
    next: *mut Node,
}

#[repr(C)]
pub struct LinkedList {
    head: *mut Node,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: std::ptr::null_mut() }
    }

    fn add_at_beginning(&mut self, value: i32) {
        let new_node = Box::into_raw(Box::new(Node {
            data: value,
            next: self.head,
        }));
        self.head = new_node;
    }

    fn add_at_end(&mut self, value: i32) {
        let new_node = Box::into_raw(Box::new(Node {
            data: value,
            next: std::ptr::null_mut(),
        }));
        unsafe {
            if self.head.is_null() {
                self.head = new_node;
                return;
            }
            let mut current = self.head;
            while !(*current).next.is_null() {
                current = (*current).next;
            }
            (*current).next = new_node;
        }
    }

    fn print_list(&self) -> String {
        let mut result = String::new();
        let mut current = self.head;
        unsafe {
            while !current.is_null() {
                result.push_str(&format!("{} -> ", (*current).data));
                current = (*current).next;
            }
        }
        result.push_str("None");
        result
    }
}

#[no_mangle]
pub extern "C" fn linked_list_new() -> *mut LinkedList {
    Box::into_raw(Box::new(LinkedList::new()))
}

#[no_mangle]
pub extern "C" fn linked_list_add_at_beginning(list: *mut LinkedList, value: i32) {
    unsafe {
        if let Some(list) = list.as_mut() {
            list.add_at_beginning(value);
        }
    }
}

#[no_mangle]
pub extern "C" fn linked_list_add_at_end(list: *mut LinkedList, value: i32) {
    unsafe {
        if let Some(list) = list.as_mut() {
            list.add_at_end(value);
        }
    }
}

#[no_mangle]
pub extern "C" fn linked_list_print(list: *mut LinkedList) -> *mut c_char {
    unsafe {
        if let Some(list) = list.as_ref() {
            let result = CString::new(list.print_list()).unwrap();
            return result.into_raw();
        }
        CString::new("List is empty").unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn linked_list_free(list: *mut LinkedList) {
    if !list.is_null() {
        unsafe { Box::from_raw(list) };
    }
}

#[no_mangle]
pub extern "C" fn linked_list_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            CString::from_raw(s);
        }
    }
}
