
pub trait Stack<T> {
    fn push(&mut self, e: T) -> &mut Self;
    fn pop(&mut self) -> Option<T>;
    fn top(&self) -> Option<&T>;
    fn top_mut(&mut self) -> Option<&mut T>;
    fn empty(&self) -> bool;
}