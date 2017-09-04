#![feature(conservative_impl_trait)]

fn add(a: u8, b: u8) -> u8 {
    a + b
}

fn sub(a: u8, b: u8) -> u8 {
    a - b
}

fn log<F, R>(f: F) -> impl Fn(u8, u8) -> u8
    where F: Fn(u8, u8) -> u8
{
    move |a, b| {
        println!("Calling with {}, {}", a, b);
        let r = f(a, b);
        println!("Result was {}", r);
        r
    }
}

fn main() {
    let logging_add = log(add);
    let logging_sub = log(sub);

    logging_add(1, 2);
    logging_sub(2, 1);
}
