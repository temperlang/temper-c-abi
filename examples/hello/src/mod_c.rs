#[no_mangle]
unsafe extern "C" fn hello_String_copy(
    selfish: *const String,
    buffer: *mut u8,
    buffer_length: usize,
) -> usize {
    let selfish = unsafe { &*selfish };
    let copy_length = selfish.len().min(buffer_length - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(selfish.as_ptr(), buffer, copy_length);
        // Null terminate.
        *buffer.add(copy_length) = 0;
    }
    copy_length
}

#[no_mangle]
unsafe extern "C" fn hello_String_free(string: *const String) {
    if !string.is_null() {
        drop(std::sync::Arc::from_raw(string));
    }
}

#[no_mangle]
unsafe extern "C" fn hello_String_length(selfish: *const String) -> usize {
    let selfish = unsafe { &*selfish };
    selfish.len()
}

#[no_mangle]
unsafe extern "C" fn hello_String_new(
    string: *const std::ffi::c_char,
    length: usize,
) -> *const String {
    let result = match length {
        0 => std::ffi::CStr::from_ptr(string).to_str(),
        _ => std::str::from_utf8(std::slice::from_raw_parts(string as *const u8, length)),
    };
    match result {
        Ok(string) => std::sync::Arc::into_raw(std::sync::Arc::new(string.to_string())),
        Err(_) => std::ptr::null(),
    }
}

#[no_mangle]
unsafe extern "C" fn hello_greeting_for(name: *const String) -> *const String {
    let name = unsafe { &*name };
    std::sync::Arc::into_raw(crate::greeting_for(name.clone()))
}
