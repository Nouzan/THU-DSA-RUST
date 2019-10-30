use lesson_linear::Vector;
use lesson_linear::vec_vector::VecVector;
use lesson_linear::list::node::ListNode;
use lesson_linear::list::{List, LessonList};
use lesson_linear::node::Node;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut a: LessonList<i32> = LessonList::default();
    a.insert_as_first(6);
    a.insert_as_first(7);
    let b = a.insert_as_last(8);
    println!("{:?}", a.remove(&a.find(&6).unwrap()));
    println!("{:?}", a.remove(&a.find(&8).unwrap()));
    println!("{:?}", a);
    println!("{:?}", b.strong_count());
}