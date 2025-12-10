pub trait DoubleMutable<T> {
    fn double_mutable(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)>;
}

impl<T> DoubleMutable<T> for Vec<T> {
    fn double_mutable(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)> {
        if a == b {
            return None;
        }

        if a < b {
            let (l, r) = self.split_at_mut(b);
            Some((&mut l[a], &mut r[0]))
        } else {
            let (l, r) = self.split_at_mut(a);
            Some((&mut r[0], &mut l[b]))
        }
    }
}
