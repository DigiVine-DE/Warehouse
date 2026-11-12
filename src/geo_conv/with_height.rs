use super::error::{PointVecChecker, VecLenErr};

pub trait To2D: Sized {
    type Height: Sized;
    fn to_2d(self) -> Result<(Self, Self::Height), VecLenErr>;
}

/// Convert 3D point to 2D point
impl To2D for Vec<f64> {
    type Height = f64;

    fn to_2d(mut self) -> Result<(Self, Self::Height), VecLenErr> {
        self.is_3d()?;
        let h = self.pop().unwrap();
        Ok((self, h))
    }
}

/// Convert an arbitrary vector consists of 3D points to a vector consists of 2D points
impl<E: To2D> To2D for Vec<E> {
    type Height = Vec<E::Height>;

    fn to_2d(self) -> Result<(Self, Self::Height), VecLenErr> {
        let len = self.len();
        let mut two_ds = Vec::with_capacity(len);
        let mut heights = Vec::with_capacity(len);
        for elem in self {
            let (two_d, h) = elem.to_2d()?;
            two_ds.push(two_d);
            heights.push(h)
        }
        Ok((two_ds, heights))
    }
}
