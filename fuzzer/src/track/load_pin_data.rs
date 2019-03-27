use angora_common::cond_stmt_base::CondStmtBase;
use angora_common::log_data::LogData;
use angora_common::tag::TagSeg;
use std::fs::File;
use std::io::{self, Read};
use std::{collections::HashMap, path::Path};

fn read_struct<T, R: Read>(mut read: R) -> io::Result<T> {
    let num_bytes = ::std::mem::size_of::<T>();
    unsafe {
        let mut s = ::std::mem::uninitialized();
        let buffer = ::std::slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
        match read.read_exact(buffer) {
            Ok(()) => Ok(s),
            Err(e) => {
                ::std::mem::forget(s);
                Err(e)
            }
        }
    }
}

fn read_vector<T, R: Read>(mut read: R, size: usize) -> io::Result<Vec<T>> {
    let mut vec = Vec::<T>::with_capacity(size);
    if size > 0 {
        unsafe {
            vec.set_len(size);
            let num_bytes = ::std::mem::size_of::<T>() * size;
            let buffer =
                ::std::slice::from_raw_parts_mut((&mut vec[..]).as_ptr() as *mut u8, num_bytes);
            match read.read_exact(buffer) {
                Ok(()) => Ok(vec),
                Err(e) => Err(e),
            }
        }
    } else {
        Ok(vec)
    }
}

pub fn get_log_data_pin(out_f: &Path) -> io::Result<LogData> {
    let mut f = match File::open(out_f) {
        Ok(file) => file,
        Err(err) => {
            panic!("could not open {:?}: {:?}", out_f, err);
        }
    };

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    // `read_exact()` comes from `Read` impl for `&[u8]`
    let mut buffer = &buffer[..];

    let num_cond = read_struct::<u32, _>(&mut buffer)? as usize;
    let num_tags = read_struct::<u32, _>(&mut buffer)? as usize;
    let num_mb = read_struct::<u32, _>(&mut buffer)? as usize;

    let cond_list = read_vector::<CondStmtBase, _>(&mut buffer, num_cond)?;
    debug!("cond_list({}): {:?}", num_cond, cond_list);

    let mut tags_map = HashMap::new();
    for _ in 0..num_tags {
        let (id, size) = read_struct::<(u32, u32), _>(&mut buffer)?;
        let offsets = read_vector::<TagSeg, _>(&mut buffer, size as usize)?;
        tags_map.insert(id, offsets);
    }
    debug!("tag_list({}): {:?}", num_tags, tags_map);

    let mut mb_map = HashMap::new();
    for _ in 0..num_mb {
        let (id, arg1_len, arg2_len) = read_struct::<(u32, u32, u32), _>(&mut buffer)?;
        let arg1 = read_vector::<u8, _>(&mut buffer, arg1_len as usize)?;
        let arg2 = read_vector::<u8, _>(&mut buffer, arg2_len as usize)?;
        mb_map.insert(id as usize, (arg1, arg2));
    }
    debug!("mb_list({}): {:?}", num_mb, mb_map);

    Ok(LogData {
        cond_list,
        tags: tags_map,
        magic_bytes: mb_map,
    })
}
