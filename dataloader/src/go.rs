use super::{graph_traffic, Float, MutableWeek};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn render_go(
    path: *const libc::c_char,
    buffer: *mut libc::c_float,
    samples: libc::size_t,
) {
    let path_cstr = unsafe { CStr::from_ptr(path) };
    let ndarray = unsafe {
        let view: &mut Float = (buffer as *mut Float).as_mut().unwrap();
        MutableWeek::from_shape_ptr((7, samples), view)
    };
    let rust_path = path_cstr.to_str().unwrap();
    graph_traffic(ndarray, samples, rust_path).unwrap();
}
