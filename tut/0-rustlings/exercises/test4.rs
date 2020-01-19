// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!



macro_rules! my_macro {
	($val: expr) => {
		if $val == "world!" {
			"Hello world!"
		} else { "" }
	};
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
   }
}
