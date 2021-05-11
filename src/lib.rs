#![link(name = "CoreML", kind = "framework")]

#[macro_use]
extern crate objc;
#[macro_use]
extern crate foreign_types;

use std::os::raw::c_void;

pub use cocoa_foundation::foundation::NSUInteger;
use objc::runtime::Object;

pub fn nsstring_as_str(nsstr: &objc::runtime::Object) -> &str {
    let bytes = unsafe {
        let bytes: *const std::os::raw::c_char = msg_send![nsstr, UTF8String];
        bytes as *const u8
    };
    let len: NSUInteger = unsafe { msg_send![nsstr, length] };
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

#[macro_export]
macro_rules! opt_nsstring_as_str {
    ($expr: expr) => {{
        #[allow(unused_assignments)]
        let mut s: *mut objc::runtime::Object = std::ptr::null_mut();
        s = $expr;
        if s.is_null() {
            None
        } else {
            Some(crate::nsstring_as_str(s.as_ref().unwrap()))
        }
    }};
}

fn nsstring_from_str(string: &str) -> *mut objc::runtime::Object {
    const UTF8_ENCODING: usize = 4;

    let cls = class!(NSString);
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut objc::runtime::Object = msg_send![cls, alloc];
        let obj: *mut objc::runtime::Object = msg_send![
            obj,
            initWithBytes:bytes
            length:string.len()
            encoding:UTF8_ENCODING
        ];
        let _: *mut c_void = msg_send![obj, autorelease];
        obj
    }
}

unsafe fn nsarray_to_vec<T: 'static>(array: *const Object) -> Vec<T> {
    let count: NSUInteger = msg_send![array, count];
    let ret = (0..count)
        .map(|i| msg_send![array, objectAtIndex: i])
        // The elements of this array are references---we convert them to owned references
        // (which just means that we increment the reference count here, and it is
        // decremented in the `Drop` impl for `Device`)
        .map(|unit: *mut Object| msg_send![unit, retain])
        .collect();
    let () = msg_send![array, release];
    ret
}

#[macro_export]
macro_rules! try_objc {
    {
        $err: ident => $body:expr
    } => {
        {
            let mut $err: *mut NSError = ::std::ptr::null_mut();
            let value = $body;
            if !$err.is_null() {
                // let desc: *mut Object = msg_send![$err_name, localizedDescription];
                // let compile_error: *const std::os::raw::c_char = msg_send![desc, UTF8String];
                // let message = CStr::from_ptr(compile_error).to_string_lossy().into_owned();
                // let () = msg_send![$err, release];
                let e = $err.as_ref().unwrap();
                return Err(e.to_owned());
                // return Err($err.as_ref().unwrap());
            }
            Ok(value)
        }
    };
}

#[inline]
fn obj_drop<T>(p: *mut T) {
    unsafe { msg_send![(p as *mut Object), release] }
}

#[inline]
fn obj_clone<T: 'static>(p: *mut T) -> *mut T {
    unsafe { msg_send![(p as *mut Object), retain] }
}

#[macro_use]
macro_rules! foreign_obj_type {
    {type CType = $raw_ident:ident;
    pub struct $owned_ident:ident;
    pub struct $ref_ident:ident;
    type ParentType = $parent_ref:ident;
    } => {
        foreign_obj_type! {
            type CType = $raw_ident;
            pub struct $owned_ident;
            pub struct $ref_ident;
        }

        impl ::std::ops::Deref for $ref_ident {
            type Target = $parent_ref;

            fn deref(&self) -> &$parent_ref {
                unsafe { &*(self as *const $ref_ident as *const $parent_ref)  }
            }
        }
    };
    {type CType = $raw_ident:ident;
    pub struct $owned_ident:ident;
    pub struct $ref_ident:ident;
    } => {
        foreign_type! {
            type CType = $raw_ident;
            fn drop = crate::obj_drop;
            fn clone = crate::obj_clone;
            pub struct $owned_ident;
            pub struct $ref_ident;
        }

        unsafe impl ::objc::Message for $raw_ident {
        }
        unsafe impl ::objc::Message for $ref_ident {
        }

        impl ::std::fmt::Debug for $ref_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                unsafe {
                    let string: *mut ::objc::runtime::Object = msg_send![self, debugDescription];
                    write!(f, "{}", crate::nsstring_as_str(&*string))
                }
            }
        }

        impl ::std::fmt::Debug for $owned_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::ops::Deref::deref(self).fmt(f)
            }
        }
    };
}

pub mod prelude;

mod ml_array_batch_provider;
pub use ml_array_batch_provider::*;

mod ml_batch_provider;
pub use ml_batch_provider::*;

mod ml_custom_layer;
pub use ml_custom_layer::*;

mod ml_custom_model;
pub use ml_custom_model::*;

mod ml_dictionary_constraint;
pub use ml_dictionary_constraint::*;

mod ml_dictionary_feature_provider;
pub use ml_dictionary_feature_provider::*;

mod ml_export;
pub use ml_export::*;

mod ml_feature_description;
pub use ml_feature_description::*;

mod ml_feature_provider;
pub use ml_feature_provider::*;

mod ml_feature_type;
pub use ml_feature_type::*;

mod ml_feature_value;
pub use ml_feature_value::*;

mod ml_feature_value_ml_image_conversion;
pub use ml_feature_value_ml_image_conversion::*;

mod ml_image_constraint;
pub use ml_image_constraint::*;

mod ml_image_size;
pub use ml_image_size::*;

mod ml_image_size_constraint;
pub use ml_image_size_constraint::*;

mod ml_image_size_constraint_type;
pub use ml_image_size_constraint_type::*;

mod ml_key;
pub use ml_key::*;

mod ml_metric_key;
pub use ml_metric_key::*;

mod ml_model;
pub use ml_model::*;

mod ml_model_collection;
pub use ml_model_collection::*;

mod ml_model_collection_entry;
pub use ml_model_collection_entry::*;

mod ml_model_configuration;
pub use ml_model_configuration::*;

mod ml_model_description;
pub use ml_model_description::*;

mod ml_model_error;
pub use ml_model_error::*;

mod ml_model_metadata_keys;
pub use ml_model_metadata_keys::*;

mod ml_model_ml_model_compilation;
pub use ml_model_ml_model_compilation::*;

mod ml_multi_array;
pub use ml_multi_array::*;

mod ml_multi_array_constraint;
pub use ml_multi_array_constraint::*;

mod ml_multi_array_shape_constraint;
pub use ml_multi_array_shape_constraint::*;

mod ml_multi_array_shape_constraint_type;
pub use ml_multi_array_shape_constraint_type::*;

mod ml_numeric_constraint;
pub use ml_numeric_constraint::*;

mod ml_parameter_description;
pub use ml_parameter_description::*;

mod ml_parameter_key;
pub use ml_parameter_key::*;

mod ml_prediction_options;
pub use ml_prediction_options::*;

mod ml_sequence;
pub use ml_sequence::*;

mod ml_sequence_constraint;
pub use ml_sequence_constraint::*;

mod ml_task;
pub use ml_task::*;

mod ml_update_context;
pub use ml_update_context::*;

mod ml_update_progress_event;
pub use ml_update_progress_event::*;

mod ml_update_progress_handlers;
pub use ml_update_progress_handlers::*;

mod ml_update_task;
pub use ml_update_task::*;

mod ml_writable;
pub use ml_writable::*;

mod ns_error;
pub use ns_error::*;

mod ns_number;
pub use ns_number::*;
