use lazy_static::lazy_static;
use libc;
use std::{collections::HashSet, sync::Mutex};

lazy_static! {
    static ref FFDS: Mutex<HashSet<u32>> = {
        let mut set = HashSet::new();
        set.insert(libc::STDIN_FILENO as u32);
        Mutex::new(set)
    };
}

#[no_mangle]
pub extern "C" fn __angora_io_add_fd(fd: libc::c_int) {
    let mut ffds = FFDS.lock().expect("Could not lock FFDS.");
    ffds.insert(fd as u32);
}

#[no_mangle]
pub extern "C" fn __angora_io_add_pfile(pfile: *mut libc::FILE) {
    let fd = unsafe { libc::fileno(pfile) };
    __angora_io_add_fd(fd);
}

#[no_mangle]
pub extern "C" fn __angora_io_remove_fd(fd: libc::c_int) {
    let mut ffds = FFDS.lock().expect("Could not lock FFDS.");
    ffds.remove(&(fd as u32));
}

#[no_mangle]
pub extern "C" fn __angora_io_remove_pfile(pfile: *mut libc::FILE) {
    let fd = unsafe { libc::fileno(pfile) };
    __angora_io_remove_fd(fd);
}

#[no_mangle]
pub extern "C" fn __angora_io_find_fd(fd: libc::c_int) -> u32 {
    let ffds = FFDS.lock().expect("Could not lock FFDS.");
    ffds.contains(&(fd as u32)) as u32
}

#[no_mangle]
pub extern "C" fn __angora_io_find_pfile(pfile: *mut libc::FILE) -> u32 {
    let fd = unsafe { libc::fileno(pfile) };
    __angora_io_find_fd(fd) as u32
}
