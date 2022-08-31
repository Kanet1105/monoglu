pub trait Data {
    fn cast_into<T>(value: T) {}
    fn cast_from<T>(value: T) {}
}