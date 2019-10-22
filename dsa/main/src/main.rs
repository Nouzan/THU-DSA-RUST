use lesson_linear::Vector;
use lesson_linear::vec_vector::VecVector;

fn main() {
    let mut v: VecVector<i32> = VecVector::new();
    let r = &mut v;
    for i in (0..100).rev() {
        r.insert_one(i);
    }
    println!("{:?}", r);
    r.sort_fully().insert_one(5).insert_one(3).remove(4, 9);
    r.sort_fully();
    println!("{:?}", r);
    println!("{}", r.search_fully(&35).unwrap());
}