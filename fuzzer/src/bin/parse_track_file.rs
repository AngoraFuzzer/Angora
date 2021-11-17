extern crate angora;
extern crate angora_common;
use angora::{cond_stmt::CondStmt, track::*};
use angora_common::defs;
use std::{env, path::PathBuf};

fn main() {
    // Usage: path_to_file output_format [pin_mode]
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("Wrong command!");
        return;
    }

    let mut output_format = "json";
    if args.len() > 2 {
        output_format = match args[2].as_str() {
            "line" => "line",
            "json_real" => "json_real",
            _ => "json",
        };
    }

    let mut pin_mode = false;
    if args.len() > 3 {
        pin_mode = match args[3].as_str() {
            "pin" => true,
            _ => false,
        };
    }

    let path = PathBuf::from(&args[1]);

    // let t = load_track_data(path.as_path(), 0, 0, 0, 0);
    let t = match read_and_parse(path.as_path(), pin_mode, false) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("parse track file error!! {:?}", err),
    };

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
    } else if output_format == "json_real" {
        let json = get_json(&t);
        println!("{}", json);
    } else {
        print!("{:#?}", t);
    }
}

pub fn get_json(t: &Vec<CondStmt>) -> String {
    match serde_json::to_string(&t) {
        Result::Ok(val) => return val,
        Result::Err(err) => panic!("Failed to serialize to json!! {:?}", err),
    };
}
