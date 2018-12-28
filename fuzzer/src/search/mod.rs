use crate::{
    cond_stmt::CondStmt,
    executor::{Executor, StatusType},
    mut_input::{self, MutInput},
};
use angora_common::config;
use rand::prelude::*;
use std::{
    self,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

mod method;
pub use self::method::*;
mod grad;
use self::grad::*;
pub mod interesting_val;
pub use self::interesting_val::*;
mod handler;
pub use self::handler::SearchHandler;

pub mod gd;
pub use self::gd::GdSearch;

pub mod random;
pub use self::random::RandomSearch;
pub mod cbh;
pub use self::cbh::CbhSearch;
pub mod mb;
pub use self::mb::MbSearch;

//Other cases of special offsets
pub mod cmpfn;
pub use self::cmpfn::FnFuzz;
pub mod len;
pub use self::len::LenFuzz;
pub mod afl;
pub use self::afl::AFLFuzz;
pub mod exploit;
pub use self::exploit::ExploitFuzz;
pub mod det;
pub use self::det::DetFuzz;
pub mod one_byte;
pub use self::one_byte::OneByteFuzz;
