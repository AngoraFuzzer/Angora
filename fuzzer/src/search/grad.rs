use angora_common::config;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(Default, Clone, Copy)]
pub struct GradUnit {
    pub sign: bool,
    pub val: u64,
    pub pct: f64,
}

impl fmt::Debug for GradUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sign {
            write!(f, "{:?}/{:?}", self.val, self.pct)
        } else {
            write!(f, "-{:?}/{:?}", self.val, self.pct)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grad(Vec<GradUnit>);

impl Grad {
    pub fn new(size: usize) -> Self {
        let mut grad = Vec::<GradUnit>::with_capacity(size);

        for _ in 0..size {
            grad.push(GradUnit::default());
        }

        Grad(grad)
    }

    pub fn normalize(&mut self) {
        // f32::MAX > u64::MAX
        let max_grad = self.max_val() as f64;
        if max_grad > 0.0 {
            for g in &mut self.0 {
                g.pct = config::GD_MOMENTUM_BETA * g.pct
                    + (1.0 - config::GD_MOMENTUM_BETA) * (g.val as f64 / max_grad);
            }
        }
    }

    pub fn max_val(&self) -> u64 {
        if let Some(g) = self.iter().max_by_key(|g| g.val) {
            g.val
        } else {
            0
        }
    }

    pub fn val_sum(&self) -> u64 {
        // self.0.iter().map(|&x| x.val).sum()
        let mut sum: u64 = 0u64;
        for x in self.0.iter() {
            sum = sum.saturating_add(x.val);
        }
        sum
    }

    pub fn clear(&mut self) {
        for g in &mut self.0 {
            g.pct = 0.0;
            g.val = 0;
        }
    }
}

impl Deref for Grad {
    type Target = Vec<GradUnit>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grad {
    //type Target = Vec<GradUnit>;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
