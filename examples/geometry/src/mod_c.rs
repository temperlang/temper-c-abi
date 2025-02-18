type RectangleRaw = *const std::sync::RwLock<crate::RectangleStruct>;

macro_rules! call_with_arc {
    ($ptr:expr, $wrap:expr, |$var:ident| $body:expr) => {{
        let $var = $wrap(std::sync::Arc::from_raw($ptr));
        let result = $body;
        std::mem::forget($var);
        result
    }};
}

#[no_mangle]
unsafe extern "C" fn geometry_Rectangle_area(selfish: RectangleRaw) -> f64 {
    // let selfish = crate::Rectangle(std::sync::Arc::from_raw(selfish));
    // let result = selfish.area();
    // std::mem::forget(selfish);
    // result
    call_with_arc!(selfish, crate::Rectangle, |selfish| selfish.area())
}

#[no_mangle]
unsafe extern "C" fn geometry_Rectangle_perimeter(selfish: RectangleRaw) -> f64 {
    call_with_arc!(selfish, crate::Rectangle, |selfish| selfish.perimeter())
}

#[no_mangle]
unsafe extern "C" fn geometry_Rectangle_to_string(selfish: RectangleRaw) -> *mut std::ffi::c_char {
    let result = call_with_arc!(selfish, crate::Rectangle, |selfish| selfish.to_string());
    std::ffi::CString::new(&**result).unwrap().into_raw()
}

#[no_mangle]
unsafe extern "C" fn geometry_Rectangle_new(width: f64, height: f64) -> RectangleRaw {
    std::sync::Arc::into_raw(crate::Rectangle::new(width, height).0)
}

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
unsafe extern "C" fn hello_String_new(string: *const std::ffi::c_char) -> *const String {
    match std::ffi::CStr::from_ptr(string).to_str() {
        Ok(string) => std::sync::Arc::into_raw(std::sync::Arc::new(string.to_string())),
        Err(_) => std::ptr::null_mut(),
    }
}
