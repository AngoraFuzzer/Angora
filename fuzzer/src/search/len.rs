// Assume it is direct and linear
use super::*;
use crate::cond_stmt::CondOutput;

pub struct LenFuzz<'a> {
    handler: SearchHandler<'a>,
}

impl<'a> LenFuzz<'a> {
    pub fn new(handler: SearchHandler<'a>) -> Self {
        Self { handler }
    }

    pub fn run(&mut self) {
        if !config::ENABLE_INPUT_LEN_EXPLORATION {
            self.handler.cond.mark_as_done();
            return;
        }

        /*
        in llvm_mode/io-func.c runtime/len_label.rs:
        lb1 => read offset
        lb2 => read size
        */
        //let offset = self.handler.cond.base.lb1 as usize;
        let size = self.handler.cond.base.lb2 as usize;
        let delta = self.handler.cond.base.get_output() as usize;
        let mut buf = self.handler.buf.clone();
        debug!(
            "len: delta {}, size: {}, buf_len: {}",
            delta,
            size,
            buf.len()
        );
        if delta > 0 {
            let extended_len = delta * size;
            if extended_len < config::MAX_INPUT_LEN {
                let buf_len = buf.len();
                if buf_len + extended_len < config::MAX_INPUT_LEN {
                    // len > X
                    let mut v = vec![0u8; extended_len + 1];
                    rand::thread_rng().fill_bytes(&mut v);
                    buf.append(&mut v);
                    self.handler.execute(&buf);
                    // some special chars: NULL, LF, CR, SPACE
                    let special_chars = vec![0, 10, 13, 32];
                    for c in special_chars {
                        buf.push(c);
                        self.handler.execute(&buf);
                        buf.pop();
                    }
                    // len == X
                    buf.pop();
                    self.handler.execute(&buf);
                }
                if buf_len > extended_len {
                    buf.truncate(buf_len - extended_len);
                    // len == X
                    self.handler.execute(&buf);
                    // len < X
                    if buf_len > extended_len + 1 {
                        buf.pop();
                        self.handler.execute(&buf);
                    }
                }
            }
        }

        self.handler.cond.mark_as_done();
    }
}
