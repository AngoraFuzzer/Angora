// Specail case for input length relative labels

use super::*;
use angora_common::{cond_stmt_base::CondStmtBase, defs};
use lazy_static::lazy_static;
use std::sync::Mutex;

const NORMAL_LABEL_WITDH: u32 = 22;
const MAX_NORMAL_LABEL: DfsanLabel = (1 << 22) - 1;
const MAX_LEN_LABEL: DfsanLabel = (1 << 10) - 1;
const NORMAL_LABEL_MASK: DfsanLabel = MAX_NORMAL_LABEL;
const NORMAL_LABEL_MASK_USIZE: usize = MAX_NORMAL_LABEL as usize;
const LEN_LABEL_MASK: DfsanLabel = MAX_LEN_LABEL;

lazy_static! {
    pub static ref LEN_INFO: Mutex<Vec<(u32, u32)>> = Mutex::new(vec![(0, 1)]);
}

pub fn is_len_label(lb: DfsanLabel) -> bool {
    lb > MAX_NORMAL_LABEL
}

pub fn get_len_label(lb: DfsanLabel) -> DfsanLabel {
    (lb >> NORMAL_LABEL_WITDH) & LEN_LABEL_MASK
}

pub fn get_normal_label(lb: DfsanLabel) -> DfsanLabel {
    lb & NORMAL_LABEL_MASK
}

pub fn get_normal_label_usize(lb: usize) -> usize {
    lb & NORMAL_LABEL_MASK_USIZE
}

pub fn get_fat_label(normal_lb: DfsanLabel, len_lb: DfsanLabel) -> DfsanLabel {
    (len_lb << NORMAL_LABEL_WITDH) | normal_lb
}

/// Function for recording offset and size
#[no_mangle]
pub extern "C" fn __angora_get_len_label(offset: u32, size: u32) -> DfsanLabel {
    let mut len_info = LEN_INFO.lock().unwrap();
    let len_lb = if len_info.len() < MAX_LEN_LABEL as usize {
        let lb = len_info.len() as DfsanLabel;
        len_info.push((offset, size));
        lb
    } else {
        0
    };
    get_fat_label(0, len_lb)
}

pub fn get_len_cond(cond: &mut CondStmtBase) -> Option<CondStmtBase> {
    let len_lb;
    if is_len_label(cond.lb1) {
        len_lb = get_len_label(cond.lb1);
        cond.lb1 = get_normal_label(cond.lb1);
    } else if is_len_label(cond.lb2) {
        len_lb = get_len_label(cond.lb2);
        cond.lb2 = get_normal_label(cond.lb2);
    } else {
        return None;
    }
    if len_lb > MAX_LEN_LABEL {
        return None;
    }
    let info = {
        let len_info = LEN_INFO.lock().unwrap();
        len_info[len_lb as usize]
    };

    let mut len_cond = *cond;
    len_cond.op = defs::COND_LEN_OP;
    len_cond.lb1 = info.0;
    len_cond.lb2 = info.1;

    Some(len_cond)
}

#[cfg(test)]
mod test {
    use super::*;
    /*
    fn get_label_normal() {
        let test_offset = 3_u32;
        let test_size = 4_u32;
        let result_label = __angora_get_len_label(test_offset, test_size);
        let info_arr = LEN_INFO.lock().unwrap();
        assert!(info_arr[0].0 == test_offset);
        assert!(info_arr[0].1 == test_size);
        assert!(result_label == 1);
    }
    */

    #[test]
    fn get_label_mult() {
        let mut input_pairs = Vec::new();
        for i in 0..10 {
            input_pairs.push(((3_u32.pow(i)) % 101, (5_u32.pow(i)) % 101));
        }
        for (i, j) in &input_pairs {
            let _result_label = __angora_get_len_label(*i, *j);
        }
        let info_arr = LEN_INFO.lock().unwrap();
        for i in 0..10 {
            assert!(
                info_arr[i + 1].0 == input_pairs[i].0,
                "0 Failed on iteration {}, {} != {}",
                i,
                info_arr[i + 1].0,
                input_pairs[i].0
            );
            assert!(
                info_arr[i + 1].1 == input_pairs[i].1,
                "1 Failed on iteration {}, {} != {}",
                i,
                info_arr[i + 1].0,
                input_pairs[i].0
            );
        }
    }

}
