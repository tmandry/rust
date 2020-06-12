#![feature(generators)]
#![feature(generator_trait)]
#![feature(test)]
use std::ops::Generator;
use std::pin::Pin;
use std::hint::black_box;

struct Big([u8; { 1024 * 1024 }]);
impl Big {
    fn new(val: u8) -> Big {
        Big([val; { 1024 * 1024 }])
    }

    fn sum(&self) -> usize {
        let mut sum = 0usize;
        for i in 0..self.0.len() {
            sum = sum.wrapping_add(self.0[i] as usize);
        }
        sum
    }
}

#[allow(unused)]
enum Kind {
    Thing1(Big),
    Thing2(Big),
    Thing3(Big),
}

fn f1(kind: Kind) -> impl Generator<Yield = usize, Return = ()> {
    move || {
        match kind {
            Kind::Thing1(arr) => yield arr.sum(),
            Kind::Thing2(arr) => yield arr.sum(),
            Kind::Thing3(arr) => yield arr.sum(),
        }
    }
}

fn main() {
    let mut gen = f1(Kind::Thing1(Big::new(1)));
    Pin::new(&mut gen).resume(());
    black_box(Kind::Thing1(Big::new(1)));
    black_box(Kind::Thing3(Big::new(2)));
    black_box(f1);
    black_box(Pin::new(&mut gen).resume(()));
}