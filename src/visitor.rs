pub trait Accept<V, R>
where
    Self: Sized,
{
    fn accept(self, visitor: &V) -> R;
}

pub trait AcceptMut<V, R>
where
    Self: Sized,
{
    fn accept(self, visitor: &mut V) -> R;
}
