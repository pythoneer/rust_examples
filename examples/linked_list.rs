#![feature(conservative_impl_trait)]

trait ListString {
    type Tail: ListString;

    fn unpack(&self) -> Option<(char, &Self::Tail)>;

    fn contains(&self, c: char) -> bool {
        match self.unpack() {
            Some((cc, _)) if cc == c => true,
            Some((_, tail)) => tail.contains(c),
            None => false,
        }
    }
}

trait StringAppend {
    type Output;

    fn append(self, c: char) -> Self::Output;
}

// CONS **************
struct Cons<T> {
    head: char,
    tail: T,
}

impl<T: ListString> ListString for Cons<T> {
    type Tail = T;

    fn unpack(&self) -> Option<(char, &Self::Tail)> {
        Some((self.head, &self.tail))
    }
}

impl<T: StringAppend> StringAppend for Cons<T> {
    type Output = Cons<T::Output>;

    fn append(self, c: char) -> Self::Output {
        Cons {
            head: self.head,
            tail: self.tail.append(c),
        }
    }
}

// NIL ***************
struct Nil;

impl ListString for Nil {
    type Tail = Nil;

    fn unpack(&self) -> Option<(char, &Self::Tail)> {
        None
    }
}

impl StringAppend for Nil {
    type Output = Cons<Self>;

    fn append(self, c: char) -> Self::Output {
        Cons {
            head: c,
            tail: self,
        }
    }
}

fn from_string<T: ListString>(sublist: T, s: String) -> impl ListString {

    let all_chars:Vec<char> = s.chars().collect();
    if all_chars.len == 0 {
        return Nil;
    } else {
        from_string(sublist)
    }

    Nil.append('b').append('a')
}

fn main() {
//    let string = Cons {
//        head: 'b',
//        tail: Cons {
//            head: 'a',
//            tail: Nil,
//        },
//    };

    //type of `string` Cons<Cons<Cons<Nil>>>
    let string = from_string(Nil,"bac".into());
    println!("size: {}", std::mem::size_of_val(&string));
    println!("contains a: {}", string.contains('a'));
}