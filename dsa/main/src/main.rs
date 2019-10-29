use lesson_linear::Vector;
use lesson_linear::vec_vector::VecVector;
// use lesson_linear::list::node::ListNode;
use lesson_linear::list::RcList;
use lesson_linear::list::List;
use lesson_linear::list::node::flat_node::{ListNode, RawNode};
use lesson_linear::node::Node;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let a = ListNode::new(1);
    a.insert_as_suss(2);
    a.insert_as_suss(3);
    a.insert_as_pred(4);
    a.suss().unwrap().insert_as_pred(5);

    println!("{:?}", a);
}