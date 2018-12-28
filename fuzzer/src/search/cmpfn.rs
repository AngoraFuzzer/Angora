use super::*;
pub struct FnFuzz<'a> {
    handler: SearchHandler<'a>,
}

impl<'a> FnFuzz<'a> {
    pub fn new(handler: SearchHandler<'a>) -> Self {
        Self { handler }
    }
    pub fn run(&mut self) {
        let mut input = self.handler.get_f_input();
        let len = self.handler.cond.variables.len() / 2;
        let output = self.handler.cond.variables.split_off(len);
        input.assign(&self.handler.cond.variables);
        self.handler.execute_input(&input);
        let input_vals = input.get_value();
        let mut has_diff = false;
        if input_vals.len() == len {
            for i in 0..len {
                let diff = output[i] as i16 - input_vals[i] as i16;
                if diff != 0 {
                    has_diff = true;
                }
                self.handler.cond.variables[i] =
                    (self.handler.cond.variables[i] as i16 - diff) as u8;
            }
        }
        if has_diff {
            input.assign(&self.handler.cond.variables);
            self.handler.execute_input(&input);
        }
        self.handler.cond.mark_as_done();
    }
}
