use lesson_linear::Vector;
use lesson_linear::vec_vector::VecVector;
use lesson_linear::list::node::ListNode;
use lesson_linear::list::{List, LessonList};
use lesson_linear::stack::Stack;
use lesson_linear::node::Node;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let a = Node::new(1);
    println!("{:?}", a.try_into_inner());
}