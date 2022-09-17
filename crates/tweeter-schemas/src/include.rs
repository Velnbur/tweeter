pub trait Include<T> {
    fn include(&mut self, resource: T) -> &mut Self;
}
