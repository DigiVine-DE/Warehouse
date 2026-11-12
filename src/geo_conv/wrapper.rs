pub trait Wrapper: Sized {
    type Inner;

    fn as_inner(&self) -> &Self::Inner;
    fn from_inner(inner: Self::Inner) -> Self;
}

pub trait JsonWrapper: Wrapper {
    fn update_bbox(&mut self, bbox: Option<Vec<f64>>);
}
