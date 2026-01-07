use std::{
    ffi::{CStr, CString, c_char},
    sync::Mutex,
};

use once_cell::sync::Lazy;

use rcheevos_hash_sys;

static RCHEEVOS_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub fn compute_hash(path: &str, buffer: Option<&[u8]>) -> Option<String> {
    let path_c = CString::new(path).ok()?;
    let mut hash = [0 as c_char; 33];

    if let Some(buffer) = buffer {
        println!("buffer: {}", buffer.len());
    } else {
        println!("no buffer");
    }

    let (ptr, len) = buffer.map_or((std::ptr::null(), 0), |b| (b.as_ptr(), b.len()));
    let mut iter: std::mem::MaybeUninit<rcheevos_hash_sys::rc_hash_iterator> =
        std::mem::MaybeUninit::uninit();

    let _guard = RCHEEVOS_MUTEX.lock().unwrap();

    unsafe {
        rcheevos_hash_sys::rc_hash_initialize_iterator(
            iter.as_mut_ptr(),
            path_c.as_ptr(),
            ptr,
            len,
        );

        let mut iter = iter.assume_init();

        // iter.consoles[0] = 78 as u8;
        // iter.index = 0;

        println!("before iterate: {} {:?}", len, path_c);
        let result = rcheevos_hash_sys::rc_hash_iterate(hash.as_mut_ptr(), &mut iter);
        println!("iterate result: {}", result);
        if result != 0 {
            let hash = CStr::from_ptr(hash.as_ptr()).to_string_lossy().into_owned();
            println!("hash: {}", hash);
            return Some(hash);
        }
    }
    None
}
