use super::*;

#[allow(dead_code)]
pub fn get_bytes_by_offsets(offsets: &Vec<TagSeg>, buf: &Vec<u8>) -> Vec<u8> {
    let mut bytes = vec![];
    for off in offsets {
        if off.begin < off.end {
            let mut v_bytes = buf[off.begin as usize..off.end as usize].to_vec();
            bytes.append(&mut v_bytes);
        }
    }
    bytes
}

pub fn set_bytes_by_offsets(offsets: &Vec<TagSeg>, bytes: &Vec<u8>, buf: &mut Vec<u8>) {
    let mut cmp_off = (0, 0);
    for off in offsets {
        if off.begin < off.end {
            cmp_off.0 = cmp_off.1;
            cmp_off.1 = cmp_off.0 + (off.end - off.begin) as usize;
            let scope = &mut buf[off.begin as usize..off.end as usize];
            scope.clone_from_slice(&bytes[cmp_off.0..cmp_off.1]);
        }
    }
}

pub fn read_val_from_buf(buf: &Vec<u8>, off: usize, size: usize) -> Result<u64, &str> {
    match size {
        1 => Ok(buf[off] as u64),
        2 => Ok(unsafe { *(&buf[off] as *const u8 as *const u16) as u64 }),
        4 => Ok(unsafe { *(&buf[off] as *const u8 as *const u32) as u64 }),
        8 => Ok(unsafe { *(&buf[off] as *const u8 as *const u64) }),
        _ => Err("strange arg off and size"),
    }
}

pub fn set_val_in_buf(buf: &mut Vec<u8>, off: usize, size: usize, val: u64) {
    match size {
        1 => {
            let v = &mut buf[off];
            *v = val as u8;
        },
        2 => {
            let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut u16) };
            *v = val as u16;
        },
        4 => {
            let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut u32) };
            *v = val as u32;
        },
        8 => {
            let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut u64) };
            *v = val as u64;
        },
        _ => {
            panic!("strange arg off and size: {}, {}", off, size);
        },
    };
}

// Optional:
// saturating_add
// overflowing_add
pub fn update_val_in_buf(
    buf: &mut Vec<u8>,
    sign: bool,
    off: usize,
    size: usize,
    direction: bool,
    delta: u64,
) {
    match size {
        1 => {
            if sign {
                let v = buf[off] as i8;
                buf[off] = if direction {
                    v.wrapping_add(delta as i8) as u8
                } else {
                    v.wrapping_sub(delta as i8) as u8
                };
            } else {
                let v = &mut buf[off];
                if direction {
                    *v = v.wrapping_add(delta as u8);
                } else {
                    *v = v.wrapping_sub(delta as u8);
                }
            }
        },
        2 => {
            if sign {
                let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut i16) };
                if direction {
                    *v = v.wrapping_add(delta as i16);
                } else {
                    *v = v.wrapping_sub(delta as i16);
                }
            } else {
                let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut u16) };
                if direction {
                    *v = v.wrapping_add(delta as u16);
                } else {
                    *v = v.wrapping_sub(delta as u16);
                }
            }
        },
        4 => {
            if sign {
                let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut i32) };
                if direction {
                    *v = v.wrapping_add(delta as i32);
                } else {
                    *v = v.wrapping_sub(delta as i32);
                }
            } else {
                let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut u32) };
                if direction {
                    *v = v.wrapping_add(delta as u32);
                } else {
                    *v = v.wrapping_sub(delta as u32);
                }
            }
        },
        8 => {
            if sign {
                let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut i64) };
                if direction {
                    *v = v.wrapping_add(delta as i64);
                } else {
                    *v = v.wrapping_sub(delta as i64);
                }
            } else {
                let v = unsafe { &mut *(&mut buf[off] as *mut u8 as *mut u64) };
                if direction {
                    *v = v.wrapping_add(delta as u64);
                } else {
                    *v = v.wrapping_sub(delta as u64);
                }
            }
        },
        _ => {
            panic!("strange arg off and size: {}, {}", off, size);
        },
    };
}
