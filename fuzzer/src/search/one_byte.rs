use super::*;

pub struct OneByteFuzz<'a> {
    pub handler: SearchHandler<'a>,
}

impl<'a> OneByteFuzz<'a> {
    pub fn new(handler: SearchHandler<'a>) -> Self {
        Self { handler }
    }

    fn execute(&mut self, input: &MutInput) {
        debug!("input : {:?}", input);
        if self.handler.cond.base.is_explore() {
            self.handler.execute_cond(input);
        } else {
            self.handler.execute_input(input);
        }
    }

    fn execute_direct(&mut self) {
        if self.handler.cond.base.is_explore() {
            self.handler.execute_cond_direct();
        } else {
            self.handler.execute_input_direct();
        }
    }
    pub fn run(&mut self) {
        if !self.handler.cond.is_first_time() {
            warn!("fuzz one byte more than one time");
            return;
        }
        self.handler.max_times = 257.into();
        let mut input = self.handler.get_f_input();
        if input.val_len() != 1 {
            error!("one byte len > 1, cond: {:?}", self.handler.cond);
            panic!();
        }
        self.execute_direct();
        for i in 0..256 {
            if self.handler.cond.is_done() {
                break;
            }
            input.set(0, i);
            self.execute(&input);
        }
    }
}
