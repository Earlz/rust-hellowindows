extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::ptr;
use std::mem;
use winapi::*;
use kernel32::*;
use user32::*;
//Conversion of C code at http://zetcode.com/gui/winapi/window/ to Rust 1.5

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{PathBuf};

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
    fn to_wide_null(&self) -> Vec<u16>;
}
impl<T> ToWide for T where T: AsRef<OsStr> {
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}
pub trait FromWide where Self: Sized {
    fn from_wide(wide: &[u16]) -> Self;
    fn from_wide_null(wide: &[u16]) -> Self {
        let len = wide.iter().take_while(|&&c| c != 0).count();
        Self::from_wide(&wide[..len])
    }
}
impl FromWide for OsString {
    fn from_wide(wide: &[u16]) -> OsString {
        OsStringExt::from_wide(wide)
    }
}
impl FromWide for PathBuf {
    fn from_wide(wide: &[u16]) -> PathBuf {
        <OsString as OsStringExt>::from_wide(wide).into()
    }
}


unsafe extern "system" fn windowproc(handle: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg{
        WM_DESTROY => {
            println!("Received quit message");
            PostQuitMessage(0);
        }
        _ => {}
    }
    return DefWindowProcW(handle, msg, wparam, lparam);
}

fn main(){
    let exit_code = program_main();
    std::process::exit(exit_code);
}

fn program_main() -> i32 {
    //get hInstance
    let hinstance:HINSTANCE = unsafe{
        let mut hmodule: HINSTANCE = ptr::null_mut();
        let GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS = 0x00000004;
        GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, "main".as_ptr() as *const i8, &mut hmodule);
        hmodule
    };
    let c="Helloworld";
    let classname=c.to_wide_null(); //We use this later as a pointer, so make sure it doesn't get thrown away
    let wc=WNDCLASSW{
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(windowproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: unsafe{LoadIconW(ptr::null_mut(), IDI_APPLICATION)},
        hCursor: unsafe{LoadCursorW(ptr::null_mut(), IDC_ARROW)},
        hbrBackground: unsafe{GetSysColorBrush(COLOR_3DFACE)},
        lpszMenuName: ptr::null_mut(),
        lpszClassName: classname.as_ptr() //OsStr::new(c).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr()
    };
    println!("Hello, world! {:p}", hinstance);
    return unsafe{
        RegisterClassW(&wc);
        let hwnd = user32::CreateWindowExW(0, wc.lpszClassName, wc.lpszClassName,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                100, 100, 350, 250, ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null_mut());
        ShowWindow(hwnd, SW_RESTORE);
        UpdateWindow(hwnd);
        let mut msg: MSG = mem::zeroed();
        while GetMessageW(&mut msg as LPMSG, ptr::null_mut(), 0, 0) != 0 {
          DispatchMessageW(&mut msg);
        }
        println!("quitting..");
        msg.wParam as i32
    };
}
