use std::{ffi::c_void, ptr::null_mut};

use windows::Win32::{
    Foundation::{GetLastError, INVALID_HANDLE_VALUE},
    System::Console::*,
};
fn main() {
    unsafe {
        let std = GetStdHandle(STD_OUTPUT_HANDLE);

        if (std == INVALID_HANDLE_VALUE) {
            //let win32err =GetLastError();

            print!("Cannot retrieve standard output handle ");
        }
        let my_num = String::from("af");
        
        //let my_num: String = "10";
        let my_num_ptr: *const String = &my_num;

       

        // const msg: &str = "hi";
        // let cstr = unsafe {CStr::from_ptr(msg as *const _)}.to_string_lossy();
         WriteConsoleW(std, my_num_ptr, 2, 2, null_mut());
    }
}
