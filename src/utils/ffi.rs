use std::slice;

use crate::{bindings::frontend::gl, traits::common::ToStr};

pub unsafe fn combine_gl_strings(
    count: gl::types::GLsizei,
    string_ptr: *const *const gl::types::GLchar,
    length_ptr: *const gl::types::GLint,
) -> String {
    if string_ptr.is_null() || count == 0 {
        return String::new();
    }

    let string_slices = unsafe {
        slice::from_raw_parts(string_ptr, count as usize)
    };

    let length_slices = if !length_ptr.is_null() {
        Some(unsafe {
            slice::from_raw_parts(length_ptr, count as usize)
        })
    } else {
        None
    };

    let mut total_bytes = 0;
    let mut rust_slices = Vec::with_capacity(count as usize);

    for i in 0..count {
        let inner_ptr = string_slices[i as usize];
        if inner_ptr.is_null() {
            continue;
        }

        let item_str: &str = match length_slices {
            Some(lens) if lens[i as usize] >= 0 => {
                let len = lens[i as usize] as usize;
                let byte_slice = unsafe {
                    slice::from_raw_parts(inner_ptr as *const u8, len)
                };
                
                str::from_utf8(byte_slice).unwrap_or("")
            }
            _ => {
                inner_ptr.to_str_or("")
            }
        };

        if !item_str.is_empty() {
            total_bytes += item_str.len();
            rust_slices.push(item_str);
        }
    }

    let mut combined_output = String::with_capacity(total_bytes);
    
    for (idx, slice) in rust_slices.iter().enumerate() {
        combined_output.push_str(slice);
    }

    combined_output
}

