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
    let a = ListNode::new(6);
    let d = a.insert_as_suss(7).insert_as_suss(8).insert_as_suss(9);

    println!("{:?}", d.pred());

    a.insert_as_suss(10);

    println!("{:?}", d.pred());
}