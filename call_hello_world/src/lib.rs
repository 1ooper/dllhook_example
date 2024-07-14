use windows::{core::PCSTR, Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA}};

#[no_mangle]
pub extern "C" fn call_hello_world() {
    println!("Call hello_world()");
    unsafe {
        let target_dll = LoadLibraryA(PCSTR(b"hello_world\0".as_ptr() as _)).unwrap();
        let target_addr = GetProcAddress(target_dll, PCSTR(b"hello_world".as_ptr() as *const u8 as _)).unwrap();
        type HelloWorld = extern "C" fn() -> ();       
        let hello_word: HelloWorld = std::mem::transmute(target_addr);
        hello_word();
    }
}
