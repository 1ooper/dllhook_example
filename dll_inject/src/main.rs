#![cfg(windows)]
#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]


use once_cell::sync::Lazy;
use retour::GenericDetour;
use std::ffi::CStr;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetProcAddress, LoadLibraryA},
    },
};
type fn_LoadLibraryA = extern "system" fn(PCSTR) -> HMODULE;

static hook_LoadLibraryA: Lazy<GenericDetour<fn_LoadLibraryA>> = Lazy::new(|| {
    let library_handle = unsafe { LoadLibraryA(PCSTR(b"kernel32.dll\0".as_ptr() as _)) }.unwrap();
    let address = unsafe { GetProcAddress(library_handle, PCSTR(b"LoadLibraryA\0".as_ptr() as _)) };
    let ori: fn_LoadLibraryA = unsafe { std::mem::transmute(address) };
    return unsafe { GenericDetour::new(ori, our_LoadLibraryA).unwrap() };
});

extern "system" fn our_LoadLibraryA(lpFileName: PCSTR) -> HMODULE {
    let file_name = unsafe { CStr::from_ptr(lpFileName.as_ptr() as _) };
    println!("our_LoadLibraryA lpFileName = {:?}", file_name);
    unsafe { hook_LoadLibraryA.disable().unwrap() };
    let my_lp_file_name = if file_name.to_str().unwrap() == "hello_world" {
        PCSTR(b"hello_world_dll\0".as_ptr())
    } else {
        lpFileName
    };
    let ret_val = hook_LoadLibraryA.call(my_lp_file_name);
    let my_lp_file_name_str = unsafe { CStr::from_ptr(my_lp_file_name.as_ptr() as _) };
    println!(
        "our_LoadLibraryA lpFileName = {:?} ret_val = {:?}",
        my_lp_file_name_str, ret_val.0
    );
    unsafe { hook_LoadLibraryA.enable().unwrap() };
    return ret_val;
}

fn main() {
    unsafe {
        let lib = libloading::Library::new("call_hello_world").unwrap();
        let call_hello_world: libloading::Symbol<unsafe extern "C" fn()> =
            lib.get(b"call_hello_world").unwrap();
        hook_LoadLibraryA.enable().unwrap();
        call_hello_world();
        hook_LoadLibraryA.disable().unwrap();
    }
}
