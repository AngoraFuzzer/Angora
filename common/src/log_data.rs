use crate::{cond_stmt_base::CondStmtMb, tag::TagSeg};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LogData {
    pub cond_list: Vec<CondStmtMb>,
    pub tags: HashMap<u32, Vec<TagSeg>>,
    pub max_offset: usize,
}

impl LogData {
    pub fn new() -> Self {
        Self {
            cond_list: vec![],
            tags: HashMap::new(),
            max_offset: 0,
        }
    }
}
