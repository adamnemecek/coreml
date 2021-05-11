use crate::prelude::*;

pub enum NSNumberFFI {}

foreign_obj_type! {
    type CType = NSNumberFFI;
    pub struct NSNumber;
    pub struct NSNumberRef;
}

impl NSNumber {
    pub fn new(v: impl Into<i64>) -> Self {
        let x = v.into();
        unsafe {
            let class = class!(NSNumber);
            msg_send![class, numberWithInteger: x]
        }
    }
}

impl NSNumberRef {}
