extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate encoding;

use winapi::winuser::*;
use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use std::ptr;
use kernel32::*;
use user32::*;
use std::slice;

use encoding::all::UTF_16LE;
use encoding::{Encoding, EncoderTrap};

//copied code from tutorial at http://zetcode.com/gui/winapi/window/

/*
MSG  msg;
HWND hwnd;
WNDCLASSW wc;

wc.style         = CS_HREDRAW | CS_VREDRAW;
wc.cbClsExtra    = 0;
wc.cbWndExtra    = 0;
wc.lpszClassName = L"Window";
wc.hInstance     = hInstance;
wc.hbrBackground = GetSysColorBrush(COLOR_3DFACE);
wc.lpszMenuName  = NULL;
wc.lpfnWndProc   = WndProc;
wc.hCursor       = LoadCursor(NULL, IDC_ARROW);
wc.hIcon         = LoadIcon(NULL, IDI_APPLICATION);

RegisterClassW(&wc);
hwnd = CreateWindowW( wc.lpszClassName, L"Window",
              WS_OVERLAPPEDWINDOW | WS_VISIBLE,
              100, 100, 350, 250, NULL, NULL, hInstance, NULL);

ShowWindow(hwnd, nCmdShow);
UpdateWindow(hwnd);

while( GetMessage(&msg, NULL, 0, 0)) {
  DispatchMessage(&msg);
}

return (int) msg.wParam;
}

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg,
  WPARAM wParam, LPARAM lParam)
{
switch(msg)
{
  case WM_DESTROY:
    PostQuitMessage(0);
    return 0;
}

return DefWindowProcW(hwnd, msg, wParam, lParam);
*/
/*
HMODULE GetCurrentModule()
{ // NB: XP+ solution!
  HMODULE hModule = NULL;
  GetModuleHandleEx(
    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
    (LPCTSTR)GetCurrentModule,
    &hModule);

  return hModule;
}
*/

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

fn main() {
    //get hInstance
    let hinstance:HINSTANCE = unsafe{
        let mut hmodule: HINSTANCE = ptr::null_mut();
        let GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS = 0x00000004;
        GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, "main".as_ptr() as *const i8, &mut hmodule);
        hmodule
    };
    let c="Helloworld";
    let classname=c.to_wide_null(); //We use this later as a pointer, so make sure it doesn't get thrown away
    let mut v: Vec<u8> = UTF_16LE.encode(c, EncoderTrap::Strict).unwrap();
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
    unsafe{
        RegisterClassW(&wc);
    }
    let hwnd = unsafe{ user32::CreateWindowExW(0, wc.lpszClassName, wc.lpszClassName,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                100, 100, 350, 250, ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null_mut())
            };
    unsafe{ShowWindow(hwnd, 9);}
    unsafe{UpdateWindow(hwnd);}
    unsafe{
        let mut msg: MSG = MSG{
            hwnd: ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT{x: 0, y: 0}
        };
        while GetMessageW(&mut msg as LPMSG, ptr::null_mut(), 0, 0) != 0 {
          DispatchMessageW(&mut msg);
        }
    }
    println!("quitting..");

}
