use super::filter;
use crate::{
    cond_stmt::{CondState, CondStmt},
    mut_input,
};
use angora_common::{defs, tag::TagSeg};
use runtime::get_log_data;
use std::{collections::HashMap, io, path::Path};

pub fn read_and_parse(
    out_f: &Path,
    enable_exploitation: bool,
) -> io::Result<(Vec<CondStmt>, usize)> {
    let (cond_list_mb, tags_map, max_offset) = {
        let data = get_log_data(out_f)?;
        (data.cond_list, data.tags, data.max_offset)
    };

    let mut cond_list: Vec<CondStmt> = Vec::new();
    // assign taint labels and magic_bytes to cond list
    for cond_mb in cond_list_mb.iter() {
        if !enable_exploitation {
            if cond_mb.base.is_exploitable() {
                continue;
            }
        }
        let mut cond = CondStmt::from(cond_mb.base);
        if cond_mb.base.op != defs::COND_LEN_OP && (cond.base.lb1 > 0 || cond.base.lb2 > 0) {
            if cond_mb.base.size == 0 {
                debug!("cond: {:?}", cond_mb.base);
            }
            get_offsets_and_variables(&tags_map, &mut cond, &cond_mb.magic_bytes);
        }

        cond_list.push(cond);
    }
    Ok((cond_list, max_offset))
}

fn get_offsets_and_variables(
    m: &HashMap<u32, Vec<TagSeg>>,
    cond: &mut CondStmt,
    magic_bytes: &Option<(Vec<u8>, Vec<u8>)>,
) {
    let empty_offsets: Vec<TagSeg> = vec![];
    let offsets1 = m.get(&cond.base.lb1).unwrap_or(&empty_offsets);
    let offsets2 = m.get(&cond.base.lb2).unwrap_or(&empty_offsets);
    if offsets2.len() == 0 || (offsets1.len() > 0 && offsets1.len() <= offsets2.len()) {
        cond.offsets = offsets1.clone();
        if cond.base.lb1 != cond.base.lb2 {
            cond.offsets_opt = offsets2.clone();
        }
        cond.variables = if let Some(args) = magic_bytes {
            [&args.1[..], &args.0[..]].concat()
        } else {
            // if it is integer comparison, we use the bytes of constant as magic bytes.
            mut_input::write_as_ule(cond.base.arg2, cond.base.size as usize)
        };
    } else {
        cond.offsets = offsets2.clone();
        cond.offsets_opt = offsets1.clone();
        cond.variables = if let Some(args) = magic_bytes {
            [&args.0[..], &args.1[..]].concat()
        //args.0.clone()
        } else {
            mut_input::write_as_ule(cond.base.arg1, cond.base.size as usize)
        };
    }
}

pub fn load_track_data(
    out_f: &Path,
    id: u32,
    speed: u32,
    enable_exploitation: bool,
) -> Vec<CondStmt> {
    let (mut cond_list, _) = match read_and_parse(out_f, enable_exploitation) {
        Result::Ok(val) => val,
        Result::Err(err) => {
            error!("parse track file error!! {:?}", err);
            (vec![], 0)
        },
    };

    for cond in cond_list.iter_mut() {
        cond.base.belong = id;
        cond.speed = speed;
        if cond.offsets.len() == 1 && cond.offsets[0].end - cond.offsets[0].begin == 1 {
            cond.state = CondState::OneByte;
        }
    }

    filter::filter_cond_list(&mut cond_list);

    cond_list
}
