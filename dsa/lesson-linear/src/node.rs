pub trait Node<T>: PartialEq + Eq + Default + Clone {
    fn new(data: T) -> Self;
    fn get(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut T>;
    fn set(&mut self, data: T) -> &mut Self;
    fn take(&mut self) -> Option<T>;
}