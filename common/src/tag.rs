use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[repr(C)] 
pub struct TagSeg {
    pub sign: bool,
    pub begin: u32,
    pub end: u32,
}

// impl TagSeg {
//     pub fn slice_from<'a>(&self, v: &'a [u8]) -> &'a [u8] {
//         &v[(self.begin as usize)..(self.end as usize)]
//     }

//     pub fn slice_from_mut<'a>(&self, v: &'a mut [u8]) -> &'a mut [u8] {
//         &mut v[(self.begin as usize)..(self.end as usize)]
//     }
// }
