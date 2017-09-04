//#![feature(associated_type_defaults)]

use std::ops::Add;
use std::borrow::Cow;

trait Sub<T> {
    type Out;

    fn baz(&self, input: Self::Out) -> Self::Out;
}

struct DataA<T> {
    a: T,
}

impl<T> Sub<T> for DataA<T> {
    fn baz(&self, input: Self::Out) -> Self::Out {
        input
    }
    type Out = ();
}

fn main() {

    let a = DataA { a: 12_i32 };

    let b = a.baz(2_i32);

}
