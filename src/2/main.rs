/*
 * Rustのスマートポインタ（Rc<T>）。
 * CreatedAt: 2019-07-05
 */
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(1,
                Rc::new(Cons(2,
                    Rc::new(Cons(3,
                        Rc::new(Nil)))))));
    println!("{}", Rc::strong_count(&a));
    let b = Cons(4, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
    {
        let c = Cons(5, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));
}

