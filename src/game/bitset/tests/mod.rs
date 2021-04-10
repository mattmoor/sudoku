use super::*;

#[test]
fn test_foreach() {
    let set = BitSet { data: 0b1111111100 };
    let mut idx = 2; // The first index is 2
    for x in set.foreach() {
        assert_eq!(x, idx);
        idx += 1;
    }
    assert_eq!(idx, 10);
}

#[test]
fn test_empty() {
    let mut set = BitSet { data: 0b11111 };
    for idx in 0..5 {
        assert!(!set.empty());
        set = set.unset(idx);
    }
    assert!(set.empty());
}

#[test]
fn test_count() {
    let mut set = BitSet { data: 0b11111 };
    let mut count = 5;
    for idx in 0..5 {
        assert_eq!(set.count(), count);
        set = set.unset(idx);
        count -= 1;
    }
    assert_eq!(set.count(), 0);
}

#[test]
fn test_singleton() {
    let mut set = BitSet { data: 0b11111 };
    for idx in 0..4 {
        assert!(set.singleton().is_none());
        set = set.unset(idx);
    }
    // We should now be a singleton of 4.
    assert_eq!(set.singleton(), Some(4));
    set = set.unset(4);

    // We are now empty.
    assert!(set.singleton().is_none());
}

#[test]
fn test_intersect() {
    let lhs = BitSet::new(&[0, 1, 2]);
    let rhs = BitSet::new(&[1, 2, 5, 6]);
    let want = BitSet::new(&[1, 2]);

    // Check our expected result.
    assert_eq!(lhs.intersect(rhs), want);
    // Check that it is a fixed point.
    assert_eq!(want.intersect(rhs), want);
    // Check that a different orientation returns something different.
    assert_ne!(want.intersect(rhs), lhs);
}

#[test]
fn test_union() {
    let lhs = BitSet::new(&[0, 1, 2]);
    let rhs = BitSet::new(&[1, 2, 5, 6]);
    let want = BitSet::new(&[0, 1, 2, 5, 6]);

    // Check our expected result.
    assert_eq!(lhs.union(rhs), want);
    // Check that it is a fixed point.
    assert_eq!(want.union(rhs), want);
}

#[test]
fn test_has() {
    let set = BitSet {
        data: 0b110110101011,
    };
    assert_eq!(set.has(0), true);
    assert_eq!(set.has(1), true);
    assert_eq!(set.has(2), false);
    assert_eq!(set.has(3), true);
    assert_eq!(set.has(4), false);
    assert_eq!(set.has(5), true);
    assert_eq!(set.has(6), false);
    assert_eq!(set.has(7), true);
    assert_eq!(set.has(8), true);
    assert_eq!(set.has(9), false);
    assert_eq!(set.has(10), true);
    assert_eq!(set.has(11), true);
}

#[test]
fn test_iterating_alternating() {
    let set = BitSet {
        data: 0b10101010101010,
    };
    let mut idx = 1; // The first index is 1
    for x in set.foreach() {
        assert_eq!(x, idx);
        idx += 2; // Every other bit is set.
    }
    assert_eq!(idx, 15);
}

#[test]
fn test_formatting() {
    assert_eq!("{}", format!("{:?}", BitSet { data: 0b0 }));
    assert_eq!("{0,1}", format!("{:?}", BitSet { data: 0b11 }));
    assert_eq!("{0,3}", format!("{:?}", BitSet { data: 0b1001 }));
    assert_eq!(
        "{1,2,3,4,5,6,7,8,9}",
        format!("{:?}", BitSet { data: 0b1111111110 })
    );
}
