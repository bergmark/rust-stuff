pub trait VecExt<T> {
    fn remove_item(&mut self, item: &T) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T>
where
    T: 'static,
    T: Eq,
{
    fn remove_item(&mut self, item: &T) -> Option<T> {
        let pos = self.iter().position(|x| *x == *item)?;
        Some(self.remove(pos))
    }
}
