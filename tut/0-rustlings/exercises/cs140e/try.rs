// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.


struct ErrorA;
struct ErrorB;

enum Error {
    A(ErrorA),
    B(ErrorB),
}

// What traits does `Error` need to implement?
impl From<ErrorA> for Error {
    fn from(errorA: ErrorA) -> Self{
        Error::A(errorA)
    }
}

impl From<ErrorB> for Error {
    fn from(errorB: ErrorB) -> Self{
        Error::B(errorB)
    }
}

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

fn do_both() -> Result<(u16, u32), Error> {
    Ok((do_a()?, do_b()?))
}

fn main() {}