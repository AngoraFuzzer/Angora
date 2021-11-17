use angora_common::tag::TagSeg;

pub fn merge_offsets(v1: &Vec<TagSeg>, v2: &Vec<TagSeg>) -> Vec<TagSeg> {
    if v1.len() == 0 {
        return v2.clone();
    }
    if v2.len() == 0 {
        return v1.clone();
    }

    let mut v = vec![];

    let mut v1_it = v1.iter();
    let mut v2_it = v2.iter();
    let mut o1 = v1_it.next();
    let mut o2 = v2_it.next();

    // The begin field is unique: if there are two different `TagSeg`, their begin field must be different.
    while o1.is_some() && o2.is_some() {
        let b1 = o1.unwrap().begin;
        let b2 = o2.unwrap().begin;
        if b1 == b2 {
            if o1.unwrap().end >= o2.unwrap().end {
                v.push(o1.unwrap().clone());
            } else {
                v.push(o2.unwrap().clone());
            }
            o1 = v1_it.next();
            o2 = v2_it.next();
        } else if b1 < b2 {
            v.push(o1.unwrap().clone());
            //merge_push(&mut v, &o1.unwrap());
            o1 = v1_it.next();
        } else {
            // b2 < b1
            v.push(o2.unwrap().clone());
            //merge_push(&mut v, &o2.unwrap());
            o2 = v2_it.next();
        }
    }

    while o1.is_some() {
        // merge_push(&mut v, &o1.unwrap());
        v.push(o1.unwrap().clone());
        o1 = v1_it.next();
    }

    while o2.is_some() {
        // merge_push(&mut v, &o2.unwrap());
        v.push(o2.unwrap().clone());
        o2 = v2_it.next();
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_offsets1() {
        let v1 = vec![TagSeg {
            sign: false,
            begin: 1,
            end: 2,
        }];
        let v2 = vec![];
        let v3 = merge_offsets(&v1, &v2);
        assert_eq!(v1[0].begin, v3[0].begin);
        assert_eq!(v1[0].end, v3[0].end);
    }

    #[test]
    fn test_merge_offsets2() {
        let v1 = vec![TagSeg {
            sign: false,
            begin: 1,
            end: 2,
        }];
        let v2 = vec![TagSeg {
            sign: false,
            begin: 1,
            end: 2,
        }];
        let v3 = merge_offsets(&v1, &v2);
        assert_eq!(v1[0].begin, v3[0].begin);
        assert_eq!(v1[0].end, v3[0].end);
    }

    #[test]
    fn test_merge_offsets3() {
        let v1 = vec![TagSeg {
            sign: false,
            begin: 1,
            end: 2,
        }];
        let v2 = vec![TagSeg {
            sign: false,
            begin: 3,
            end: 4,
        }];
        let v3 = merge_offsets(&v1, &v2);
        assert_eq!(v1[0].begin, v3[0].begin);
        assert_eq!(v1[0].end, v3[0].end);
        assert_eq!(v2[0].begin, v3[1].begin);
        assert_eq!(v2[0].end, v3[1].end);
    }

    #[test]
    fn test_merge_offsets4() {
        let v1 = vec![
            TagSeg {
                sign: false,
                begin: 1,
                end: 2,
            },
            TagSeg {
                sign: false,
                begin: 3,
                end: 4,
            },
            TagSeg {
                sign: false,
                begin: 10,
                end: 14,
            },
        ];
        let v2 = vec![
            TagSeg {
                sign: false,
                begin: 3,
                end: 4,
            },
            TagSeg {
                sign: false,
                begin: 5,
                end: 6,
            },
        ];
        let v3 = merge_offsets(&v1, &v2);
        let vt = vec![
            TagSeg {
                sign: false,
                begin: 1,
                end: 2,
            },
            TagSeg {
                sign: false,
                begin: 3,
                end: 4,
            },
            TagSeg {
                sign: false,
                begin: 5,
                end: 6,
            },
            TagSeg {
                sign: false,
                begin: 10,
                end: 14,
            },
        ];
        assert_eq!(v3.len(), vt.len());
        for (a, b) in vt.iter().zip(v3.iter()) {
            assert_eq!(a.begin, b.begin);
            assert_eq!(a.end, b.end);
        }
    }

    #[test]
    fn test_merge_offsets5() {
        let v1 = vec![TagSeg {
            sign: false,
            begin: 0,
            end: 2,
        }];
        let v2 = vec![TagSeg {
            sign: false,
            begin: 0,
            end: 4,
        }];
        let v3 = merge_offsets(&v1, &v2);
        assert_eq!(v2[0].begin, v3[0].begin);
        assert_eq!(v2[0].end, v3[0].end);
    }

    #[test]
    fn test_merge_offsets6() {
        let v1 = vec![TagSeg {
            sign: false,
            begin: 0,
            end: 4,
        }];
        let v2 = vec![TagSeg {
            sign: false,
            begin: 2,
            end: 6,
        }];
        let _v3 = merge_offsets(&v1, &v2);
        // assert_eq!(v1[0].begin, v3[0].begin);
        // assert_eq!(v2[0].begin, v3[0].end);
        // assert_eq!(v2[0].begin, v3[1].begin);
        // assert_eq!(v1[0].end, v3[1].end);
        // assert_eq!(v1[0].end, v3[2].begin);
        // assert_eq!(v2[0].end, v3[2].end);
    }
}
