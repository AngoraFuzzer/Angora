const VEC_CAP: usize = 1 << 16;
const LABEL_WITDH: u32 = 22;
const MAX_LB: usize = (1 << LABEL_WITDH) - 1;
const ROOT: usize = 0;

use angora_common::tag::TagSeg;
// TODO : cache https://github.com/jaemk/cached/blob/master/src/stores.rs

#[derive(Debug)]
struct TagNode {
    left: usize,
    right: usize,
    parent: usize,
    and_op: bool,
    seg: TagSeg,
}

impl TagNode {
    pub fn new(parent: usize, begin: u32, end: u32) -> Self {
        Self {
            left: 0,
            right: 0,
            parent,
            and_op: false,
            seg: TagSeg {
                sign: false,
                begin,
                end,
            },
        }
    }

    #[inline(always)]
    pub fn get_seg_size(&self) -> u32 {
        self.seg.end - self.seg.begin
    }
}

pub struct TagSet {
    nodes: Vec<TagNode>,
}

impl TagSet {
    pub fn new() -> Self {
        let mut nodes = Vec::with_capacity(VEC_CAP);
        nodes.push(TagNode::new(ROOT, 0, 0));
        Self { nodes }
    }

    #[inline(always)]
    fn new_node(&mut self, parent: usize, begin: u32, end: u32) -> usize {
        let lb = self.nodes.len();
        if lb < MAX_LB {
            self.nodes.push(TagNode::new(parent, begin, end));
            lb
        } else {
            eprintln!("[ERR] more than {} nodes..", MAX_LB);
            panic!();
        }
    }

    fn insert_n_zeros(&mut self, mut cur_lb: usize, num: u32, last_one_lb: usize) -> usize {
        let mut n = num;

        while n != 0 {
            let next = self.nodes[cur_lb].left;
            let next_size = self.nodes[next].get_seg_size();
            if next == 0 {
                // leaf node
                let off = self.nodes[cur_lb].seg.end;
                let new_lb = self.new_node(last_one_lb, off, off + n);
                self.nodes[cur_lb].left = new_lb;
                cur_lb = new_lb;
                n = 0;
            } else if next_size > n {
                // split
                let off = self.nodes[cur_lb].seg.end;
                let new_lb = self.new_node(last_one_lb, off, off + n);
                self.nodes[cur_lb].left = new_lb;
                self.nodes[new_lb].left = next;
                self.nodes[next].seg.begin = off + n;
                cur_lb = new_lb;
                n = 0;
            } else {
                // next_size <= n
                cur_lb = next;
                n -= next_size
            }
        }
        cur_lb
    }

    fn insert_n_ones(&mut self, mut cur_lb: usize, num: u32, mut last_one_lb: usize) -> usize {
        let mut n = num;
        while n != 0 {
            let next = self.nodes[cur_lb].right;
            let last_end = self.nodes[cur_lb].seg.end;
            if next == 0 {
                // leaf node
                let off = last_end;
                let new_lb = self.new_node(last_one_lb, off, off + n);
                self.nodes[cur_lb].right = new_lb;
                cur_lb = new_lb;
                n = 0;
            } else {
                let next_end = self.nodes[next].seg.end;
                // to avoid next_lb is a group bytes..
                // e.g [0, 1], [0, 10],   [0, 10] is grouped though it is after [0, 1]
                // its size should be 10 - 1 instead of 10 - 0
                // if we insert 3 ones after [0, 1],
                // it should be [0, 1], [1, 4], [0, 10]
                // if it is [0, 1], [1, 10], insert 3 ones after [0, 1] => [0, 1], [1, 4], [1, 10]
                let next_size = next_end - last_end; // get_seg_size();
                if next_size > n {
                    // split
                    let off = last_end;
                    let new_lb = self.new_node(last_one_lb, off, off + n);
                    self.nodes[cur_lb].right = new_lb;
                    self.nodes[new_lb].right = next;
                    self.nodes[next].parent = new_lb;
                    self.nodes[next].seg.begin = off + n;
                    cur_lb = new_lb;
                    n = 0;
                } else {
                    // next_size <= n
                    cur_lb = next;
                    n -= next_size;
                }
            }
            last_one_lb = cur_lb; // update last one lb
        }
        cur_lb
    }

    pub fn insert(&mut self, offset: u32) -> usize {
        let mut cur_lb = self.insert_n_zeros(ROOT, offset, ROOT);
        cur_lb = self.insert_n_ones(cur_lb, 1, ROOT);
        cur_lb
    }

    pub fn set_sign(&mut self, lb: usize) {
        self.nodes[lb].seg.sign = true;
    }

    pub fn get_sign(&self, lb: usize) -> bool {
        if lb < self.nodes.len() {
            self.nodes[lb].seg.sign
        } else {
            false
        }
    }

    pub fn combine_and(&mut self, lb: usize) {
        self.nodes[lb].and_op = true;
    }

    pub fn split_and_op(&mut self, lb: usize) -> usize {
        // if the lb combine some other constants with and
        // e.g. x && 0xFF,
        // We break up its shape (if it has)
        // if x is [0, 4], then x becomes [0, 1], [1, 2], [2, 3], [3, 4]
        // eprintln!("combine_and {:?}, {:?}", lb, self.nodes[lb]);
        let seg = self.nodes[lb].seg;
        if seg.end - seg.begin > 1 {
            let p = self.nodes[lb].parent;
            if p != ROOT && self.nodes[p].seg.begin >= seg.begin {
                if self.nodes[p].seg.end < seg.end {
                    // group by : [0, 1], [0, 4]
                    // we transelate it to  [0, 1], [1, 2], [2, 3], [0, 4]
                    let mut cur_lb = p;
                    while cur_lb != lb {
                        cur_lb = self.insert_n_ones(cur_lb, 1, cur_lb);
                    }
                }
                self.nodes[lb].seg.begin = seg.end - 1;
            }
        }
        lb
    }

    /*
    If the program access some continuous bytes in one instruction, we can view these bytes as a group.
    They may be represented as a single variable: such as short, int.
    Here we check the labels of the first and last bytes, if their content(seg) only has one offset, and
    their distance(Offset_last - Offset_first) is equal to the number of bytes,
    then they should be grouped.
    Here, we assume that the byte orders of the variable between memory and file are the same.
    */
    fn infer_shape(&mut self, l1: usize, l2: usize, len: usize) -> Option<usize> {
        if len != 2 && len != 4 && len != 8 {
            return None;
        }
        let len = len as u32;
        // assume l1 < l2
        if self.nodes[l1].parent == ROOT
            && self.nodes[l2].parent == ROOT
            && self.nodes[l1].seg.begin + len == self.nodes[l2].seg.end
        {
            let cur_lb = self.insert_n_ones(l1, len - 1, l1);

            // FIXME: be careful
            // if it is not grouped by a shape : it will be [0, 1], [1, 2], [2, 3], [3, 4]
            // otherwise, it will be [0, 1], [1, 2], [2, 3], [0, 4],
            // or [0, 1], [0, 4]
            // The last begin of last seg is 0!
            self.nodes[cur_lb].seg.begin = self.nodes[l1].seg.begin;
            // println!("infer: {:?}, size: {:?}, ->  cur {:?}", self.nodes[l1], len, self.nodes[cur_lb]);
            Some(cur_lb)
        } else {
            None
        }
    }

    // infer shape in mathmatical operation, e.g add, sub
    pub fn infer_shape2(&mut self, lb: usize, len: usize) {
        if lb == ROOT || self.nodes[lb].seg.begin + 1 < self.nodes[lb].seg.end {
            return;
        }
        let mut cur_lb = lb;
        for _ in 0..(len - 1) {
            cur_lb = self.nodes[cur_lb].parent;

            if cur_lb == ROOT {
                return;
            }
        }
        if self.nodes[cur_lb].parent == ROOT {
            if self.nodes[cur_lb].seg.begin + len as u32 == self.nodes[lb].seg.end {
                self.nodes[lb].seg.begin = self.nodes[cur_lb].seg.begin;
            }
        }
    }

    // load inst will only call combine_n if size >= 2
    pub fn combine_n(&mut self, lbs: Vec<usize>, infer: bool) -> usize {
        let mut lb_iter = lbs.iter();
        let mut cur_lb = lb_iter.next();

        while cur_lb == Some(&ROOT) {
            cur_lb = lb_iter.next();
        }

        if cur_lb.is_none() {
            return ROOT;
        }

        let mut cur_lb = *cur_lb.unwrap();
        let last_lb = *lbs.last().unwrap();

        let mut next_lb = lb_iter.next();
        if next_lb.is_none() {
            return cur_lb;
        }

        if infer {
            if let Some(lb) = self.infer_shape(cur_lb, last_lb, lbs.len()) {
                return lb;
            }
        }

        while next_lb.is_some() {
            cur_lb = self.combine(cur_lb, *next_lb.unwrap());
            next_lb = lb_iter.next();
        }

        cur_lb
    }

    pub fn combine(&mut self, mut l1: usize, mut l2: usize) -> usize {
        if l1 == 0 {
            return l2;
        }
        if l2 == 0 || l1 == l2 {
            return l1;
        }

        if l1 > l2 {
            // swap to make sure l1 <= l2
            std::mem::swap(&mut l2, &mut l1);
        }

        let mut lb_st = vec![];

        let mut last_begin = MAX_LB as u32;
        while l1 > 0 && l1 != l2 {
            let b1 = self.nodes[l1].seg.begin;
            let b2 = self.nodes[l2].seg.begin;
            if b1 < b2 {
                if b2 < last_begin {
                    lb_st.push(l2);
                    last_begin = b2;
                }
                l2 = self.nodes[l2].parent;
            } else {
                if b1 < last_begin {
                    lb_st.push(l1);
                    last_begin = b1;
                }
                l1 = self.nodes[l1].parent;
            }
        }

        let mut cur_lb = if l1 > 0 { l1 } else { l2 };

        while !lb_st.is_empty() {
            let cur_seg = self.nodes[cur_lb].seg;
            let next = lb_st.pop().unwrap();
            let next_seg = self.nodes[next].seg;
            if cur_seg.end >= next_seg.begin {
                if next_seg.end > cur_seg.end {
                    cur_lb = self.insert_n_ones(cur_lb, next_seg.end - cur_seg.end, cur_lb);
                }
            } else {
                // has gap
                let last_lb = cur_lb;
                let gap = next_seg.begin - cur_seg.end;
                cur_lb = self.insert_n_zeros(cur_lb, gap, last_lb);
                let size = next_seg.end - next_seg.begin;
                cur_lb = self.insert_n_ones(cur_lb, size, last_lb);
            }

            if next_seg.sign {
                self.nodes[cur_lb].seg.sign = true;
            }
        }

        cur_lb
    }

    pub fn find(&mut self, mut lb: usize) -> Vec<TagSeg> {
        // assert!(lb < self.nodes.len());
        let mut tag_list = vec![];
        let mut last_begin = MAX_LB as u32;
        while lb > 0 {
            if self.nodes[lb].and_op {
                lb = self.split_and_op(lb);
            }
            let t = self.nodes[lb].seg;
            if t.begin < last_begin {
                tag_list.push(t);
                last_begin = t.begin;
            }
            lb = self.nodes[lb].parent;
        }

        if tag_list.len() > 1 {
            tag_list.reverse();
        }

        tag_list
    }

    pub fn get_num_nodes(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_set_tests_basic() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 0..20 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }

        assert_eq!(tag_set.combine(0, 0), 0);
        assert_eq!(tag_set.combine(lbs[1], 0), lbs[1]);
        assert_eq!(tag_set.combine(0, lbs[10]), lbs[10]);
        assert_eq!(tag_set.combine(lbs[3], lbs[3]), lbs[3]);

        let list = tag_set.find(lbs[1]);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 1,
                end: 2
            }
        );

        let l = tag_set.combine(lbs[1], lbs[2]);
        let list = tag_set.find(l);
        assert_eq!(list.len(), 2);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 1,
                end: 2
            }
        );
        assert_eq!(
            list[1],
            TagSeg {
                sign: false,
                begin: 2,
                end: 3
            }
        );

        let l2 = tag_set.combine(l, lbs[3]);
        let list = tag_set.find(l2);
        assert_eq!(list.len(), 3);
        assert_eq!(
            list[2],
            TagSeg {
                sign: false,
                begin: 3,
                end: 4
            }
        );

        let l = tag_set.combine(lbs[2], lbs[5]);
        let list = tag_set.find(l);
        assert_eq!(list.len(), 2);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 2,
                end: 3
            }
        );
        assert_eq!(
            list[1],
            TagSeg {
                sign: false,
                begin: 5,
                end: 6
            }
        );
    }

    #[test]
    fn tag_set_tests_group_variable1() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 0..20 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }

        let l2 = tag_set.combine_n(lbs[0..3].to_vec(), true);
        let list = tag_set.find(l2);
        assert_eq!(list.len(), 3);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 1
            }
        );
        assert_eq!(
            list[1],
            TagSeg {
                sign: false,
                begin: 1,
                end: 2
            }
        );
        assert_eq!(
            list[2],
            TagSeg {
                sign: false,
                begin: 2,
                end: 3
            }
        );

        let l1 = tag_set.combine_n(lbs[0..4].to_vec(), true);
        let list = tag_set.find(l1);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );

        let l3 = tag_set.combine_n(lbs[2..10].to_vec(), true);
        let list = tag_set.find(l3);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 2,
                end: 10
            }
        );

        let l4 = tag_set.combine_n(lbs[6..10].to_vec(), true);
        let list = tag_set.find(l4);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 6,
                end: 10
            }
        );

        let _l5 = tag_set.combine(l3, l4);
        let list = tag_set.find(l3);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 2,
                end: 10
            }
        );

        let l6 = tag_set.combine(l1, l4);
        let list = tag_set.find(l6);
        // println!("list : {:?}", list);
        assert_eq!(list.len(), 2);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );
        assert_eq!(
            list[1],
            TagSeg {
                sign: false,
                begin: 6,
                end: 10
            }
        );

        // They should has 2 variable!!
        let l7 = tag_set.combine_n(lbs[2..6].to_vec(), true);
        let list = tag_set.find(l7);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 2,
                end: 6
            }
        );
        let l8 = tag_set.combine(l7, l4);

        let list = tag_set.find(l4);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 6,
                end: 10
            }
        );

        let list = tag_set.find(l8);
        assert_eq!(list.len(), 2);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 2,
                end: 6
            }
        );
        assert_eq!(
            list[1],
            TagSeg {
                sign: false,
                begin: 6,
                end: 10
            }
        );
    }

    #[test]
    fn tag_set_tests_group_variable2() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 0..20 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }

        let l1 = tag_set.combine_n(lbs[0..4].to_vec(), true);
        let list = tag_set.find(l1);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );

        let l2 = tag_set.combine(l1, lbs[1]);
        let list = tag_set.find(l2);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );
        let l3 = tag_set.combine(l2, lbs[2]);
        let list = tag_set.find(l3);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );
        let l4 = tag_set.combine(l3, lbs[3]);
        let list = tag_set.find(l4);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );
    }

    #[test]
    fn tag_set_tests_group_variable3() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 0..20 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }

        let mut l1 = lbs[0];
        for i in 1..20 {
            l1 = tag_set.combine(l1, lbs[i]);
        }

        let l2 = tag_set.combine_n(lbs[0..4].to_vec(), true);
        let list = tag_set.find(l2);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );

        let l3 = tag_set.combine_n(lbs[0..4].to_vec(), true);
        let list = tag_set.find(l3);
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 0,
                end: 4
            }
        );
    }

      #[test]
    fn tag_set_tests_infer_shape() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 2..6 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }

        let l1 = tag_set.combine_n(lbs, true);
        let list = tag_set.find(l1);

        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 2,
                end: 6
            }
        );
    }

    #[test]
    fn tag_set_tests_infer_shape2() {
        // https://github.com/AngoraFuzzer/Angora/pull/50
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 2..6 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }

        let l1 = tag_set.combine_n(lbs, false);

        let list = tag_set.find(l1);

        assert_eq!(list.len(), 4);
        assert_eq!(
            list[3],
            TagSeg {
                sign: false,
                begin: 5,
                end: 6
            }
        );

        tag_set.infer_shape2(l1, 4);
        let list = tag_set.find(l1);

        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0],
            TagSeg {
                sign: false,
                begin: 2,
                end: 6
            }
        );
    }

    #[test]
    fn tag_set_tests_group_sign() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 0..20 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }
        tag_set.set_sign(lbs[1]);
        let list = tag_set.find(lbs[1]);
        assert_eq!(
            list[0],
            TagSeg {
                sign: true,
                begin: 1,
                end: 2
            }
        );
    }

    #[test]
    fn tag_set_tests_loop() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        let n: u32 = 2000;
        for i in 0..n {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }
        let mut lb = lbs[0];
        let mut len = 1;
        for i in 1..n {
            lb = tag_set.combine(lb, lbs[i as usize]);
            let list = tag_set.find(lb);
            assert_eq!(len + 1, list.len());
            len += 1;
        }
    }

    #[test]
    fn tag_set_tests_loop2() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        let mut lens = vec![];
        let n: u32 = 2000;
        for i in 0..n {
            let lb = tag_set.insert(i);
            lbs.push(lb);
            lens.push(1);
        }
        for i in 0..10000 {
            let l1: usize = (i * 3) % lbs.len();
            let l2: usize = (i * 7) % lbs.len();
            let lb = tag_set.combine(lbs[l1], lbs[l2]);
            let list = tag_set.find(lb);
            let len = list.len();

            if len < lens[l1] || len < lens[l2] {
                println!("lb :  {}, {}, {}", lb, lbs[l1], lbs[l2]);
                println!("len:  {}, {}, {}", len, lens[l1], lens[l2]);
                println!("l0: {:?}", list);
                println!("l1: {:?}", tag_set.find(lbs[l1]));
                println!("l2: {:?}", tag_set.find(lbs[l2]));
            }

            assert!(len >= lens[l1]);
            assert!(len >= lens[l2]);
            lbs.push(lb);
            lens.push(len);
        }
    }

    #[test]
    fn tag_set_tests_split_and_op() {
        let mut tag_set = TagSet::new();
        let mut lbs = vec![];
        for i in 0..8 {
            let lb = tag_set.insert(i);
            lbs.push(lb);
        }

        let lb = tag_set.combine_n(lbs[0..8].to_vec(), true);
        let list = tag_set.find(lb);
        assert_eq!(list.len(), 1);
        let lb = tag_set.split_and_op(lb);
        let list = tag_set.find(lb);
        assert_eq!(list.len(), 8);
    }

}
