use std::borrow::Borrow;

// // Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct MyStruct<'a> {
    x: &'a str,
}

// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a MyStruct, y: &'b i32) {
    println!("x is {:?} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
// fn failed_borrow<'a>() {
//     let _x = 12;
//
//     // ERROR: `_x` does not live long enough
//     let _y: &'a i32 = &_x;
//     // Attempting to use the lifetime `'a` as an explicit type annotation
//     // inside the function will fail because the lifetime of `&_x` is shorter
//     // than that of `_y`. A short lifetime cannot be coerced into a longer one.
// }

fn main() {
    // Create variables to be borrowed below.
    let four = 4;
    let my_string = "my_string".to_string();
    let my_struct = MyStruct { x: &my_string };

    // Borrows (`&`) of both variables are passed into the function.
    {
        let nine = 9;
        print_refs(&my_struct, &nine);
    }
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    // failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}

