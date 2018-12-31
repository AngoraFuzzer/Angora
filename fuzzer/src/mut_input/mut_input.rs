use super::*;
use crate::search;
use angora_common::config;

use rand::{self, distributions::Uniform, Rng};

use std::{fmt, u8};
//use std::u16;
use std::{cmp, u32, u64};

#[derive(Clone, Debug, Constructor)]
struct InputMeta {
    sign: bool,
    offset: usize,
    size: usize,
}

#[derive(Clone)]
pub struct MutInput {
    value: Vec<u8>,
    meta: Vec<InputMeta>,
}

impl MutInput {
    pub fn new() -> Self {
        Self {
            value: vec![],
            meta: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.meta.len()
    }

    pub fn val_len(&self) -> usize {
        self.value.len()
    }

    pub fn from(offsets: &Vec<TagSeg>, input: &Vec<u8>) -> Self {
        let len = input.len();
        let mut mut_input = MutInput::new();
        for off in offsets {
            let begin = off.begin as usize;
            let end = off.end as usize;
            if begin == end {
                continue;
            }
            if end <= len {
                mut_input.push((input[begin..end]).to_vec(), off.sign);
            } else {
                // end > len
                if begin >= len {
                    let r = end - begin;
                    mut_input.push(vec![0u8; r], off.sign);
                } else {
                    // begin < len
                    let mut v = input[begin..len].to_vec();
                    let r = end - len;
                    let mut ext_v = vec![0u8; r];
                    v.append(&mut ext_v);
                    mut_input.push(v, off.sign);
                }
            }
        }

        mut_input
    }

    // ATT: ele will be moved
    fn push(&mut self, mut ele: Vec<u8>, sign: bool) {
        if ele.len() != 1 && ele.len() != 2 && ele.len() != 4 && ele.len() != 8 {
            for _ in 0..ele.len() {
                self.meta.push(InputMeta::new(sign, self.value.len(), 1));
            }
        } else {
            self.meta
                .push(InputMeta::new(sign, self.value.len(), ele.len()));
        }
        self.value.append(&mut ele);
    }

    pub fn update(&mut self, index: usize, direction: bool, delta: u64) {
        let info = &self.meta[index];
        update_val_in_buf(
            &mut self.value,
            info.sign,
            info.offset,
            info.size,
            direction,
            delta,
        );
    }

    // the return value is unsigned!!
    pub fn get_entry(&self, index: usize) -> u64 {
        let info = &self.meta[index];
        match read_val_from_buf(&self.value, info.offset, info.size) {
            Ok(v) => v,
            Err(_) => {
                panic!("meta: {:?}", self.meta);
            },
        }
    }

    pub fn get_entry_len(&self, index: usize) -> usize {
        self.meta[index].size
    }

    pub fn set(&mut self, index: usize, val: u64) {
        let info = &self.meta[index];
        set_val_in_buf(&mut self.value, info.offset, info.size, val);
    }

    pub fn assign(&mut self, val: &Vec<u8>) {
        let l = cmp::min(val.len(), self.val_len());
        if l > 0 {
            let scope = &mut self.value[0..l];
            scope.clone_from_slice(&val[0..l]);
        }
    }

    pub fn get_value(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn set_value_from_input(&mut self, input: &MutInput) {
        self.value = input.get_value();
    }

    pub fn bitflip(&mut self, i: usize) {
        let byte_i = i >> 3;
        let bit_i = i & 7;
        assert!(byte_i < self.val_len());
        self.value[byte_i] ^= 128 >> bit_i;
    }

    pub fn write_to_input(&self, offsets: &Vec<TagSeg>, input: &mut Vec<u8>) {
        //assert_eq!(self.len(), offsets.len());
        if offsets.len() > 0 {
            let ext_len = offsets.last().unwrap().end as usize;
            let orig_len = input.len();
            if ext_len > orig_len {
                let mut v = vec![0u8; ext_len - orig_len];
                input.append(&mut v);
            }
        }
        set_bytes_by_offsets(offsets, &self.value, input);
    }

    pub fn randomize_all(&mut self) {
        let mut rng = rand::thread_rng();
        self.randomize_all_with_weight(&mut rng, 3);
    }

    pub fn randomize_all_with_weight<T: Rng>(&mut self, rng: &mut T, weight: u32) {
        // 1/weight true
        let coin = rng.gen_bool(1.0 / weight as f64);
        if coin {
            self.randomize_all_uniform(rng);
        } else {
            self.randomize_all_mut_based(rng);
        }
    }

    pub fn randomize_all_uniform<T: Rng>(&mut self, rng: &mut T) {
        rng.fill_bytes(&mut self.value);
    }

    pub fn randomize_all_mut_based<T: Rng>(&mut self, rng: &mut T) {
        let entry_len = self.len() as u32;
        let byte_len = self.val_len() as u32;
        assert!(byte_len > 0 && entry_len > 0);

        let use_stacking = if byte_len <= 4 {
            1 + rng.gen_range(0, 16)
        } else if byte_len <= 20 {
            1 + rng.gen_range(0, 64)
        } else {
            1 + rng.gen_range(0, 256)
        };

        // let choice_range = Range::new(0, 6);
        let choice_range = Uniform::new(0, 6);

        for _ in 0..use_stacking {
            match rng.sample(choice_range) {
                0 | 1 => {
                    // flip bit
                    let byte_idx: u32 = rng.gen_range(0, byte_len);
                    let bit_idx: u32 = rng.gen_range(0, 8);
                    self.value[byte_idx as usize] ^= 128 >> bit_idx;
                },
                2 => {
                    //add
                    let entry_idx: u32 = rng.gen_range(0, entry_len);
                    let v: u32 = rng.gen_range(1, config::MUTATE_ARITH_MAX);
                    self.update(entry_idx as usize, true, v as u64);
                },
                3 => {
                    // sub
                    let entry_idx: u32 = rng.gen_range(0, entry_len);
                    let v: u32 = rng.gen_range(1, config::MUTATE_ARITH_MAX);
                    self.update(entry_idx as usize, false, v as u64);
                },
                4 => {
                    // set interesting value
                    let entry_idx: u32 = rng.gen_range(0, entry_len as u32);
                    let n = self.get_entry_len(entry_idx as usize);
                    let vals = search::get_interesting_bytes(n);
                    let wh = rng.gen_range(0, vals.len() as u32);
                    self.set(entry_idx as usize, vals[wh as usize]);
                },
                5 => {
                    // random byte
                    let byte_idx: u32 = rng.gen_range(0, byte_len);
                    // self.randomize_one_byte(byte_idx as usize);
                    self.value[byte_idx as usize] = rng.gen();
                },
                _ => {},
            }
        }
    }
}

impl fmt::Debug for MutInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{}, ", self.get_entry(i))?
        }
        Ok(())
    }
}
