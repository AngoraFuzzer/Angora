mod mut_input;
pub mod offsets;
mod rw;
mod serialize;

use angora_common::tag::TagSeg;

pub use self::{mut_input::MutInput, rw::*, serialize::*};
