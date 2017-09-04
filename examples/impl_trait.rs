#![feature(conservative_impl_trait)]

trait TheThing {
    fn do_it(&self) -> u32;
}

struct DataType(u32);

impl TheThing for DataType {
    fn do_it(&self) -> u32 {
        self.0
    }
}

fn implTraitFunction() -> impl TheThing {
    DataType(23)
}

fn getIterator() -> impl Iterator<Item = i32> {
    (0..100).take(20).map(|x| x * 2)
}

fn adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn main() {

    let a = adder(2);
    println!("add: {}", a(3));
    println!("add: {}", a(1));
    let b = adder(8);
    println!("add: {}", b(3));
    println!("add: {}", adder(12)(13));
    println!("add: {}", adder(100)(300));

    let x = implTraitFunction();
    println!("test: {}", x.do_it());

    for y in getIterator() {
        println!("y: {}", y);
    }
}
