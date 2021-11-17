use byteorder::{LittleEndian, WriteBytesExt};

// pub unsafe fn repr_as_raw_bytes<T: Sized>(p: &T) -> &[u8] {
//     std::slice::from_raw_parts(
//         (p as *const T) as *const u8,
//         std::mem::size_of::<T>(),
//     )
// }

// pub unsafe fn repr_as_raw_bytes_mut<T: Sized>(p: &mut T) -> &mut [u8] {
//     std::slice::from_raw_parts_mut((p as *const T) as *mut u8, std::mem::size_of::<T>())
// }

pub fn write_as_ule(val: u64, size: usize) -> Vec<u8> {
    let mut wtr = vec![];
    match size {
        1 => {
            wtr.write_u8(val as u8).unwrap();
        },
        2 => {
            wtr.write_u16::<LittleEndian>(val as u16).unwrap();
        },
        4 => {
            wtr.write_u32::<LittleEndian>(val as u32).unwrap();
        },
        8 => {
            wtr.write_u64::<LittleEndian>(val as u64).unwrap();
        },
        _ => {
            debug!("wrong size: {:?}", size);
            // panic!("strange arg size: {}", size);
        },
    }

    wtr
}

// pub fn read_as_ule(buf: &Vec<u8>, size: usize) -> u64 {
//     let mut rdr = Cursor::new(buf);
//     match size {
//         1 => rdr.read_u8().unwrap() as u64,
//         2 => rdr.read_u16::<LittleEndian>().unwrap() as u64,
//         4 => rdr.read_u32::<LittleEndian>().unwrap() as u64,
//         _ => rdr.read_u64::<LittleEndian>().unwrap() as u64,
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write_as_ule() {
        let n: u32 = 1934642260;
        let v = write_as_ule(n as u64, 4);
        println!("{:?}", v);
        assert!(v.len() == 4);
    }
}
