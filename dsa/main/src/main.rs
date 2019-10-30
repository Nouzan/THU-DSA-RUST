use lesson_linear::Vector;
use lesson_linear::vec_vector::VecVector;
use lesson_linear::list::node::ListNode;
use lesson_linear::list::{List, LessonList};
use lesson_linear::stack::Stack;
use lesson_linear::node::Node;

use std::rc::Rc;
use std::cell::RefCell;


fn main() {
    let mut A: LessonList<ListNode<i32>> = LessonList::new();
    let mut a = ListNode::new(3);
    A.push(a.clone());
    println!("{:?}", A);
    a.take();
    println!("{:?}", A);
    a.set(15);
    println!("{:?}", A);
    let x = A.top_mut().unwrap();
    x.set(19);
    println!("{:?}", A);
    a.set(18);
    println!("{:?}", A);
}