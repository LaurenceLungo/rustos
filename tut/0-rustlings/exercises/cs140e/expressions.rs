// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

// I AM NOT DONE

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

// Implement a generic function here
fn max<U,T>(a: U, b: T) {
    
}

#[test]
fn expressions() {
    assert_eq!(max(1usize, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(IntWrapper(120), IntWrapper(248)), IntWrapper(248));
}
