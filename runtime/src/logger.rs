use bincode::{deserialize_from, serialize_into};
use std::{collections::HashMap, env, fs, io, path::Path};

use crate::{len_label, tag_set_wrap};
use angora_common::{cond_stmt_base::CondStmtMb, config, defs, log_data::LogData};

#[derive(Debug)]
pub struct Logger {
    data: LogData,
    fd: Option<fs::File>,
    order_map: HashMap<(u32, u32), u32>,
}

impl Logger {
    pub fn new() -> Self {
        // export ANGORA_TRACK_OUTPUT=track.log
        let fd = match env::var(defs::TRACK_OUTPUT_VAR) {
            Ok(path) => match fs::File::create(&path) {
                Ok(f) => Some(f),
                Err(_) => None,
            },
            Err(_) => None,
        };

        Self {
            data: LogData::new(),
            fd,
            order_map: HashMap::new(),
        }
    }

    fn save_tag(&mut self, lb: u32) {
        if lb > 0 {
            let tag = tag_set_wrap::tag_set_find(lb as usize);
            if let Some(v) = tag.last() {
                if v.end as usize > self.data.max_offset {
                    self.data.max_offset = v.end as usize;
                }
            }
            self.data.tags.entry(lb).or_insert(tag);
        }
    }

    // like the fn in fparser.rs
    pub fn get_order(&mut self, cond: &mut CondStmtMb) -> u32 {
        let order_key = (cond.base.cmpid, cond.base.context);
        let order = self.order_map.entry(order_key).or_insert(0);
        if cond.base.order == 0 {
            // first case in switch
            let order_inc = *order + 1;
            *order = order_inc;
        }
        cond.base.order += *order;
        *order
    }

    pub fn save(&mut self, mut cond: CondStmtMb) {
        if cond.base.lb1 == 0 && cond.base.lb2 == 0 {
            return;
        }

        let mut order = 0;

        // also modify cond.base to remove len_label information
        let len_cond = len_label::get_len_cond(&mut cond.base);

        if cond.base.op < defs::COND_AFL_OP || cond.base.op == defs::COND_FN_OP {
            self.save_tag(cond.base.lb1);
            self.save_tag(cond.base.lb2);
            order = self.get_order(&mut cond);
        }
        if order <= config::MAX_COND_ORDER {
            self.data.cond_list.push(cond);

            if let Some(mut c) = len_cond {
                c.order = 0x10000 + order; // avoid the same as cond;
                self.data.cond_list.push(CondStmtMb {
                    base: c,
                    magic_bytes: None,
                });
            }
        }
    }

    fn fini(&self) {
        if let Some(fd) = &self.fd {
            let mut writer = io::BufWriter::new(fd);
            serialize_into(&mut writer, &self.data).expect("Could not serialize data.");
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.fini();
    }
}

pub fn get_log_data(path: &Path) -> io::Result<LogData> {
    let f = fs::File::open(path)?;
    let mut reader = io::BufReader::new(f);
    match deserialize_from::<&mut io::BufReader<fs::File>, LogData>(&mut reader) {
        Ok(v) => Ok(v),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "bincode parse error!")),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
