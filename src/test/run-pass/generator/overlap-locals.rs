#![feature(generators)]

fn main() {
    let a = || {
        {
            let x: i32 = 5;
            yield;
            println!("{:?}", x);
        }
        {
            let y: i32 = 6;
            yield;
            println!("{:?}", y);
        }
    };
    assert_eq!(8, std::mem::size_of_val(&a));
}
