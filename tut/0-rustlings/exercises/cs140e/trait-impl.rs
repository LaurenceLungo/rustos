// FIXME: Make me pass! Diff budget: 25 lines.


enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl PartialEq for Duration {
    fn eq(&self, rhs: &Self) -> bool {
        use Duration::*;
        match (self, rhs) {
            (MilliSeconds(a), MilliSeconds(b)) => a == b,
            (MilliSeconds(a), Seconds(b)) => *a == *b as u64 *1000,
            (MilliSeconds(a), Minutes(b)) => *a == *b as u64 *60*1000,
            (Seconds(a), MilliSeconds(b)) => *a as u64 *1000 == *b,
            (Seconds(a), Seconds(b)) => a == b,
            (Seconds(a), Minutes(b)) => *a == *b as u32 *60,
            (Minutes(a), Minutes(b)) => a == b,
            (Minutes(a), Seconds(b)) => *a as u32 *60 == *b,
            (Minutes(a), MilliSeconds(b)) => *a as u64 *60*1000 == *b,
        }
    }
}

// What traits does `Duration` need to implement?

#[test]
fn traits() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}

pub fn main() {}