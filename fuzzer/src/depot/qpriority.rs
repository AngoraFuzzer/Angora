use angora_common::defs;
use std::{self, cmp::Ordering, fmt};

const INIT_PRIORITY: u16 = 0;
const AFL_INIT_PRIORITY: u16 = 0;
const DONE_PRIORITY: u16 = std::u16::MAX;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct QPriority(u16);
impl QPriority {
    pub fn inc(&self, op: u32) -> Self {
        if op == defs::COND_AFL_OP {
            self.afl_inc()
        } else {
            self.base_inc()
        }
    }
    fn base_inc(&self) -> Self {
        QPriority(self.0 + 1)
    }

    fn afl_inc(&self) -> Self {
        QPriority(self.0 + 2)
    }

    pub fn init(op: u32) -> Self {
        if op == defs::COND_AFL_OP {
            Self::afl_init()
        } else {
            Self::base_init()
        }
    }

    fn base_init() -> Self {
        QPriority(INIT_PRIORITY)
    }

    fn afl_init() -> Self {
        QPriority(AFL_INIT_PRIORITY)
    }

    pub fn done() -> Self {
        QPriority(DONE_PRIORITY)
    }

    pub fn is_done(&self) -> bool {
        self.0 == DONE_PRIORITY
    }
}

// Make the queue get smallest priority first.
impl Ord for QPriority {
    fn cmp(&self, other: &QPriority) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl PartialOrd for QPriority {
    fn partial_cmp(&self, other: &QPriority) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for QPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
