extern crate angora;
extern crate angora_common;
use angora::track::*;
use angora_common::defs;
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("Wrong command!");
        return;
    }

    let path = PathBuf::from(&args[1]);

    // let t = load_track_data(path.as_path(), 0, 0, 0, 0);
    let t = match read_and_parse(path.as_path(), true, false) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("parse track file error!! {:?}", err),
    };

    let mut output_format = "json";
    if args.len() > 2 {
        if args[2] == "line" {
            output_format = "line";
        }
    }

    if output_format == "line" {
        for cond in t {
            // println!("{:?}", cond.base);
            let op = cond.base.op;
            if (op & defs::COND_BASIC_MASK) == defs::COND_SW_OP {
                // println!("SW: cmpid {}, context {}, order{}, condition {}",
                // cond.base.cmpid, cond.base.context, cond.base.order, cond.base.condition);
            } else if cond.base.is_explore() {
                println!(
                    "CMP: cmpid {}, context {}, args ({}, {}), condition {}",
                    cond.base.cmpid,
                    cond.base.context,
                    cond.base.arg1,
                    cond.base.arg2,
                    cond.base.condition
                );
            }
        }
    } else {
        print!("{:#?}", t);
    }
}
