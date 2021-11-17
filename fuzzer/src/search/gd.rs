/*
Gradient descend search
*/

use super::*;
use std;

pub struct GdSearch<'a> {
    handler: SearchHandler<'a>,
    sample_index: (usize, usize),
}

impl<'a> GdSearch<'a> {
    pub fn new(handler: SearchHandler<'a>) -> Self {
        Self {
            handler,
            sample_index: (0, 0),
        }
    }

    fn execute(&mut self, input: &MutInput) -> u64 {
        if self.handler.skip {
            return self.handler.executor.last_f;
        }
        debug!("input : {:?}", input);
        let f = self.handler.execute_cond(input);
        f
    }

    fn random_fuzz<T: Rng>(&mut self, rng: &mut T) {
        let mut fmin = std::u64::MAX;
        let mut input = self.handler.get_f_input();
        let mut input_min = input.get_value();
        loop {
            if self.handler.is_stopped_or_skip() {
                break;
            }
            input.assign(&input_min);
            input.randomize_all_with_weight(rng, 3);
            let f0 = self.execute(&input);
            if f0 < fmin {
                fmin = f0;
                input_min = input.get_value();
                break;
            }
        }
        if fmin < std::u64::MAX {
            self.handler.cond.variables = input_min;
        }
    }

    pub fn run<T: Rng>(&mut self, rng: &mut T) {
        let mut input = self.handler.get_f_input();
        assert!(
            input.len() > 0,
            "Input length == 0!! {:?}",
            self.handler.cond
        );

        let mut f0 = if !self.handler.cond.is_first_time() {
            self.reload_input(&mut input)
        } else {
            self.handler.cond.linear = true;
            self.init_start_point(&mut input)
        };

        if f0 == std::u64::MAX {
            self.random_fuzz(rng);
            return;
        }

        let mut ep_i = 0;
        let mut grad = Grad::new(input.len());
        loop {
            trace!(">>> epoch={}, f0={}", ep_i, f0);
            if self.handler.is_stopped_or_skip() {
                break;
            }

            self.cal_gradient(&input, f0, &mut grad);

            let mut g_i = 0;
            while grad.max_val() == 0 {
                if self.handler.is_stopped_or_skip() || g_i > config::MAX_NUM_MINIMAL_OPTIMA_ROUND {
                    break;
                }
                debug!("Stuck in minor optima! g_i={} f0={}", g_i, f0);
                self.handler.cond.num_minimal_optima += 1;
                g_i += 1;
                f0 = self.repick_start_point(&mut input, f0, rng);
                debug!("repick {:?}", input);
                if self.handler.is_stopped_or_skip() {
                    break;
                }
                grad.clear();
                self.cal_gradient(&input, f0, &mut grad);
            }

            if self.handler.is_stopped_or_skip() || g_i > config::MAX_NUM_MINIMAL_OPTIMA_ROUND {
                break;
            }

            grad.normalize();

            trace!("input={:?}, gradient={:?}", input, grad);
            f0 = self.descend(&mut input, f0, &grad, rng);
            ep_i += 1;
        }

        if self.handler.executor.last_f < std::u64::MAX {
            self.handler.cond.variables = input.get_value();
        }
    }

    fn reload_input(&mut self, input_min: &mut MutInput) -> u64 {
        input_min.assign(&self.handler.cond.variables);
        self.execute(&input_min)
    }

    fn init_start_point(&mut self, input_min: &mut MutInput) -> u64 {
        debug!("Init start...");
        let mut input = input_min.clone();
        let mut fmin = self.handler.execute_cond_direct();

        input.assign(&self.handler.cond.variables);
        let f1 = self.execute(&input);
        if f1 < fmin {
            fmin = f1;
            input_min.set_value_from_input(&input);
        }
        // reverse endian
        if f1 > 1 {
            let mut rev_v = self.handler.cond.variables.clone();
            rev_v.reverse();
            input.assign(&rev_v);
            let f1 = self.execute(&input);
            if f1 < fmin {
                fmin = f1;
                input_min.set_value_from_input(&input);
            }
        }

        fmin
    }

    fn get_interesting_point(&mut self, input: &mut MutInput) -> bool {
        if self.handler.cond.is_first_time() && self.sample_index.0 < input.len() {
            let n = input.get_entry_len(self.sample_index.0);
            if self.sample_index.1 < n {
                let interesting_vals = get_interesting_bytes(n);
                input.set(self.sample_index.0, interesting_vals[self.sample_index.1]);

                self.sample_index.1 += 1;
                if self.sample_index.1 == n {
                    self.sample_index.1 = 0;
                    self.sample_index.0 += 1;
                }
                return true;
            }
        }
        false
    }

    fn repick_start_point<T: Rng>(
        &mut self,
        input_min: &mut MutInput,
        _f0: u64,
        rng: &mut T,
    ) -> u64 {
        let mut fmin = std::u64::MAX;
        let mut input = input_min.clone();

        // for _ in 0..config::MAX_RANDOM_SAMPLE_NUM {
        loop {
            if self.handler.is_stopped_or_skip() {
                break;
            }

            let has_int_p = self.get_interesting_point(&mut input);
            if !has_int_p {
                // input.randomize_all_with_weight(rng, 2);
                input.randomize_all_uniform(rng);
            }

            let f1 = self.execute(&input);
            if f1 < fmin {
                fmin = f1;
                input_min.set_value_from_input(&input);
                break;
            }

            if has_int_p {
                input = input_min.clone();
            }
        }

        fmin
    }
    // @return: sign, is_linear, det
    fn partial_derivative(
        &mut self,
        orig_input: &MutInput,
        i: usize,
        f0: u64,
    ) -> (bool, bool, u64) {
        let mut input = orig_input.clone();
        let orig_val = input.get_entry(i);
        input.update(i, true, 1);
        let f_plus = self.execute(&input);
        input.set(i, orig_val);
        input.update(i, false, 1);
        let f_minus = self.execute(&input);

        // trace!("f0={} plus={} minus={}", f0, f_plus, f_minus);
        match (f_minus < f0, f_plus < f0) {
            (false, false) => (true, false, 0), // no gradient
            (false, true) => (
                true,
                f_minus != f0 && f_minus - f0 == f0 - f_plus,
                f0 - f_plus,
            ),
            (true, false) => (
                false,
                f_plus != f0 && f0 - f_minus == f_plus - f0,
                f0 - f_minus,
            ),
            (true, true) => {
                if f_minus < f_plus {
                    (false, false, f0 - f_minus)
                } else {
                    (true, false, f0 - f_plus)
                }
            },
        }
    }

    fn cal_gradient(&mut self, input: &MutInput, f0: u64, grad: &mut Grad) {
        debug!("start calculate gradient.. input {:?}, f0 {:?}", input, f0);
        // let mut grad = Grad::new(input.len());
        // grad.mul(config::GD_MOMENTUM_BETA);
        let mut max = 0_u64;
        for (i, g) in grad.iter_mut().enumerate() {
            if self.handler.is_stopped_or_skip() {
                break;
            }
            let (s, l, f) = self.partial_derivative(input, i, f0);
            if f > max {
                max = f;
            }
            self.handler.cond.linear = self.handler.cond.linear && l;
            g.sign = s;
            g.val = f;
            // g.add(s, (f as f64 * (1 - config::GD_MOMENTUM_BETA) as u64));
        }
    }

    fn compute_delta_all(input: &mut MutInput, grad: &Grad, step: usize) {
        let step = step as f64;
        for (i, g) in grad.iter().enumerate() {
            let movement = g.pct * step;
            input.update(i, g.sign, movement as u64);
        }
    }

    fn descend(
        &mut self,
        input_min: &mut MutInput,
        f0: u64,
        grad: &Grad,
        rng: &mut impl Rng,
    ) -> u64 {
        let mut f_last = f0;
        let mut input = input_min.clone();
        let mut delta_index: Option<usize> = None;

        debug!("descend..");

        let vsum = grad.val_sum();
        if vsum > 0 {
            let guess_step = f0 / vsum;
            debug!(
                "f0 is : {}, vsum: {}, input: {:?}, guess step is : {}",
                f0, vsum, input, guess_step
            );
            Self::compute_delta_all(&mut input, grad, guess_step as usize);
            let f_new = self.execute(&input);
            if f_new >= f_last {
                input.set_value_from_input(&input_min);
            } else {
                input_min.set_value_from_input(&input);
                f_last = f_new;
            }
        }

        let mut step: usize = 1;

        loop {
            loop {
                if self.handler.is_stopped_or_skip() {
                    return f_last;
                }
                if let Some(idx) = delta_index {
                    // only descent by idx-th dimension
                    let movement = (grad[idx].pct * step as f64).round() as u64;
                    input.update(idx, grad[idx].sign, movement);
                } else {
                    // all dimensions
                    Self::compute_delta_all(&mut input, grad, step);
                };

                let f_new = self.execute(&input);

                trace!("step={:?} f_new={:?}", step, f_new);

                // until the gradient is not work
                // or f_new > f_last
                if f_new >= f_last {
                    if f_new == std::u64::MAX || rng.gen_bool(config::GD_ESCAPE_RATIO) {
                        break;
                    }
                }

                step *= 2;
                input_min.set_value_from_input(&input);
                f_last = f_new;
            }

            if grad.len() == 1 {
                break;
            } else {
                // only use one grad, iterate all index
                let mut new_idx = if let Some(idx) = delta_index {
                    idx + 1
                } else {
                    0
                };
                // find an proper dimension
                while new_idx < grad.len() && grad[new_idx].pct < 0.01 {
                    new_idx += 1;
                }

                if new_idx >= grad.len() {
                    break;
                }
                assert!(grad[new_idx].pct > 0.0);
                debug!(
                    "switch to one dem: idx:{}, pct:{}",
                    new_idx, grad[new_idx].pct
                );
                delta_index = Some(new_idx);
                input.set_value_from_input(&input_min);
                step = 1;
            }
        }

        trace!("update input: f0= {}, f_last={}, step={}", f0, f_last, step);

        f_last
    }
}
