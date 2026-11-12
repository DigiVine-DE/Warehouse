use std::iter::FromIterator;

use super::error::VecLenErr;

pub trait ToVec<Elem> {
    fn to_vec(&self) -> Vec<Elem>;
}

// Vec<ToVec<elem>> -> Vec<Vec<elem>>
impl<T, Elem> ToVec<Vec<Elem>> for Vec<T>
where
    T: ToVec<Elem>,
{
    fn to_vec(&self) -> Vec<Vec<Elem>> {
        self.iter().map(|x| x.to_vec()).collect()
    }
}

pub trait TryFromVecRef<Elem>: Sized {
    fn try_from_vec_ref(v: &Vec<Elem>) -> Result<Self, VecLenErr>;
}

/// Generate a default vector for a geo type
pub trait GeoDefaultVec: Sized + Default {
    fn geo_default_vec() -> Vec<Self>;

    fn geo_default_vec_3d() -> Vec<Self> {
        Self::geo_default_vec()
    }
}

impl GeoDefaultVec for f64 {
    fn geo_default_vec() -> Vec<Self> {
        // 2D Point
        vec![0., 0.]
    }

    fn geo_default_vec_3d() -> Vec<Self> {
        // 3D Point
        vec![0., 0., 0.]
    }
}

impl<Elem> GeoDefaultVec for Vec<Elem> {
    fn geo_default_vec() -> Vec<Self> {
        // all geo collections
        vec![]
    }
}

/// A marker trait for blanket implementing TryFromVecRef
pub trait GeoCollection {
    /// the element geo type of this GeoCollection
    type GeoItem;
}

/// if a geo collection can be constructed by the iterator of child items, and the child items can be constructed by a vector of grandchild items
/// then the geo collection can be constructed by Vec<Vec<grandchild>>
impl<Elem, T> TryFromVecRef<Vec<Elem>> for T
where
    T: GeoCollection + FromIterator<T::GeoItem>,
    T::GeoItem: TryFromVecRef<Elem>,
{
    fn try_from_vec_ref(v: &Vec<Vec<Elem>>) -> Result<Self, VecLenErr> {
        let geo = v
            .iter()
            .map(T::GeoItem::try_from_vec_ref)
            .collect::<Result<Self, _>>()?;
        Ok(geo)
    }
}
