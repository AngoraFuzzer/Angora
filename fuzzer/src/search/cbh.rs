// Climb Hill.
use super::*;

pub struct CbhSearch<'a> {
    handler: SearchHandler<'a>,
}

impl<'a> CbhSearch<'a> {
    pub fn new(handler: SearchHandler<'a>) -> Self {
        Self { handler }
    }

    pub fn run(&mut self) {
        let mut input = self.handler.get_f_input();
        assert!(
            input.len() > 0,
            "Input length < 0!! {:?}",
            self.handler.cond
        );
        let mut fmin = self.handler.execute_cond(&input);
        let mut input_min = input.get_value();

        if input.val_len() == self.handler.cond.variables.len() {
            input.assign(&self.handler.cond.variables);
            let f = self.handler.execute_cond(&input);
            if f < fmin {
                fmin = f;
                input_min = input.get_value();
            }
        }

        loop {
            if self.handler.is_stopped_or_skip() {
                break;
            }
            input.assign(&input_min);
            input.randomize_all();
            let f0 = self.handler.execute_cond(&input);
            if f0 < fmin {
                fmin = f0;
                input_min = input.get_value();
            }
        }

        self.handler.cond.variables = input_min;
    }
}
