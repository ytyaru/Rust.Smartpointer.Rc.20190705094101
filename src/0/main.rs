/*
 * Rustのスマートポインタ（Box<T>）。
 * CreatedAt: 2019-07-05
 */
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let a = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    let b = Cons(4, Box::new(a));
    let c = Cons(5, Box::new(a));
//    println!("{:?}", list);
}

