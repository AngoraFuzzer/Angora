use crate::{len_label::*, tag_set::TagSet};
use angora_common::{config, tag::TagSeg};
use lazy_static::lazy_static;
use std::{slice, sync::Mutex};

// Lazy static doesn't have reference count and won't call drop after the program finish.
// So, we should call drop manually.. see ***_fini.
lazy_static! {
    static ref TS: Mutex<Option<TagSet>> = Mutex::new(Some(TagSet::new()));
}

#[no_mangle]
pub extern "C" fn __angora_tag_set_insert(offset: u32) -> u32 {
    let mut tsl = TS.lock().unwrap();
    if let Some(ref mut ts) = *tsl {
        ts.insert(offset) as u32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn __angora_tag_set_combine(mut lb1: u32, mut lb2: u32) -> u32 {
    let mut len_lb = 0;
    if is_len_label(lb1) {
        lb1 = get_normal_label(lb1);
        len_lb = get_len_label(lb1);
    }
    if is_len_label(lb2) {
        lb2 = get_normal_label(lb2);
        len_lb = get_len_label(lb2);
    }

    let mut tsl = TS.lock().unwrap();
    let lb = if let Some(ref mut ts) = *tsl {
        ts.combine(lb1 as usize, lb2 as usize) as u32
    } else {
        0
    };

    get_fat_label(lb, len_lb)
}

#[no_mangle]
pub extern "C" fn __angora_tag_set_combine_n(lbs: *const u32, size: u32, infer_shape: bool) -> u32 {
    let mut len_lb = 0;
    let lbs = unsafe { slice::from_raw_parts(lbs, size as usize) };
    let lbs = lbs
        .iter()
        .map(|l| {
            if is_len_label(*l) {
                let lb = get_normal_label(*l);
                len_lb = get_len_label(*l);
                lb as usize
            } else {
                *l as usize
            }
        })
        .collect::<Vec<usize>>();
    let mut tsl = TS.lock().unwrap();
    let lb = if let Some(ref mut ts) = *tsl {
        ts.combine_n(lbs, infer_shape) as u32
    } else {
        0
    };

    get_fat_label(lb, len_lb)
}

// called in dfsan/pass/DFSanPass
#[no_mangle]
pub extern "C" fn __angora_tag_set_mark_sign(lb: u32) {
    let mut tsl = TS.lock().unwrap();
    if let Some(ref mut ts) = *tsl {
        ts.set_sign(get_normal_label_usize(lb as usize));
    }
}

#[no_mangle]
pub extern "C" fn __angora_tag_set_infer_shape_in_math_op(lb: u32, len: u32) {
    let mut tsl = TS.lock().unwrap();
    if let Some(ref mut ts) = *tsl {
        ts.infer_shape2(get_normal_label_usize(lb as usize), len as usize);
    }
}

// called in dfsan/pass/DFSanPass
#[no_mangle]
pub extern "C" fn __angora_tag_set_combine_and(lb: u32) {
    if config::DISABLE_INFER_SHAPE_IF_HAS_AND_OP {
        let mut tsl = TS.lock().unwrap();
        if let Some(ref mut ts) = *tsl {
            ts.combine_and(get_normal_label_usize(lb as usize));
        }
    }
}

#[no_mangle]
pub extern "C" fn __angora_tag_set_fini() {
    let mut tsl = TS.lock().unwrap();
    *tsl = None;
}

pub fn tag_set_find(lb: usize) -> Vec<TagSeg> {
    let mut tsl = TS.lock().unwrap();
    if let Some(ref mut ts) = *tsl {
        ts.find(get_normal_label_usize(lb))
    } else {
        vec![]
    }
}

pub fn tag_set_get_sign(lb: usize) -> bool {
    let tsl = TS.lock().unwrap();
    if let Some(ref ts) = *tsl {
        ts.get_sign(get_normal_label_usize(lb))
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn __angora_tag_set_show(lb: usize) {
    println!("{:?}", tag_set_find(lb));
}
