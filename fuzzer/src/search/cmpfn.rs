use super::*;
use angora_common::tag::TagSeg;
pub struct FnFuzz<'a> {
    handler: SearchHandler<'a>,
}

impl<'a> FnFuzz<'a> {
    pub fn new(handler: SearchHandler<'a>) -> Self {
        Self { handler }
    }

    fn insert_bytes(&mut self, n: usize) {
        debug!("add {} bytes", n);
        let last = self.handler.cond.offsets.last().unwrap();
        let off = last.begin as usize;
        let mut end = last.end;
        if self.handler.buf.len() <= last.end as usize {
            self.handler.buf.resize(last.end as usize + 1_usize, 0);
        }
        let v = self.handler.buf[off];
        for _ in 0..n {
            self.handler.buf.insert(off, v);
            let begin = end;
            end = begin + 1;
            self.handler.cond.offsets.push(TagSeg {
                sign: false,
                begin,
                end,
            })
        }
    }

    fn remove_bytes(&mut self, n: usize) {
        debug!("remove {} bytes", n);
        for _ in 0..n {
            let last = self.handler.cond.offsets.last().unwrap();
            let off = last.begin as usize;
            let size = last.end as usize - off;
            self.handler.buf.remove(off);
            if size > 1 {
                self.handler.cond.offsets.last_mut().unwrap().end = last.end - 1;
            } else {
                self.handler.cond.offsets.pop();
            }
        }
    }

    pub fn run(&mut self) {
        let input = self.handler.get_f_input();
        let len = self.handler.cond.base.size as usize; // magic bytes's length
        if len > self.handler.cond.variables.len() {
            error!(
                "maigic length is less than input length. cond: {:?}",
                self.handler.cond
            );
            return;
        }
        let output = self.handler.cond.variables.split_off(len); // mapping input
        let input_len = input.val_len();
        if input_len < len {
            self.insert_bytes(len - input_len);
        } else if input_len > len {
            self.remove_bytes(input_len - len);
        }

        let mut input = self.handler.get_f_input();
        let input_vals = input.get_value();
        // input_vals.len() becomes len now.
        assert_eq!(input_vals.len(), len);
        let min_len = std::cmp::min(len, output.len());
        assert!(min_len <= self.handler.cond.variables.len());
        for i in 0..min_len {
            let diff = output[i] as i16 - input_vals[i] as i16;
            self.handler.cond.variables[i] = (self.handler.cond.variables[i] as i16 - diff) as u8;
        }

        input.assign(&self.handler.cond.variables);
        self.handler.execute_input(&input);

        self.handler.cond.mark_as_done();
    }
}
