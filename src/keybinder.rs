use libc::size_t;
use libc::c_char;
use libc::c_int;
use libc::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std;

#[link(name = "keybinder-3.0")]
extern {
    fn keybinder_init();
    fn keybinder_bind(keystring: *const c_char, handler: unsafe extern fn(*const c_char, *mut c_void), user_data: *mut c_void) -> c_int;
    fn keybinder_unbind(keystring: *const c_char, handler: unsafe extern fn(*const c_char, *mut c_void));
    fn keybinder_unbind_all(keystring: *const c_char);
    fn keybinder_get_current_event_time() -> u32;
}

unsafe extern fn handler<F>(keycode: *const c_char, arg: *mut c_void) where F: FnMut() {
    let keycode = CStr::from_ptr(keycode).to_str();
    match keycode {
        Ok(keycode) => {
            //todo pass keycode to closure
            let closure = arg as *mut F;
            (*closure)();
        },
        Err(_) => {
            println!("Utf8 error for {:?}", keycode)
        }
    }
}

pub fn bind<F: Fn()>(hotkey: &str, callback: F) -> bool where F: FnMut() {
    let c_msg = std::ffi::CString::new(hotkey).unwrap();

    let cb = &callback as *const _ as *mut c_void;
    unsafe { keybinder_bind(c_msg.as_ptr(), handler::<F>, cb) == 1 }
}

pub fn init() {
    unsafe { keybinder_init(); }
}