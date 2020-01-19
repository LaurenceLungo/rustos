// FIXME: Make me compile! Diff budget: 2 lines.



struct MyStruct(usize);

impl MyStruct {
    fn make_1(&mut self) {
        self.0 = 1;
    }
}

pub fn main() {
    let x = &mut MyStruct(10);
    x.make_1();
}
