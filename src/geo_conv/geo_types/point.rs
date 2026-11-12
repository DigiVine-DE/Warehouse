use super::*;

use geo::coord;

type R = geo::Coord;
type J2 = models::Point;
type J3 = models::PointWithHeight;
type Elem = f64;
type V = Vec<Elem>;

impl_json_wrapper!(J2, V, "Point");
impl_json_wrapper!(J3, V, "Point");

impl ToVec<Elem> for gps::Point {
    fn to_vec(&self) -> Vec<Elem> {
        vec![self.x, self.y]
    }
}

impl ToVec<Elem> for gps::PointZ {
    fn to_vec(&self) -> Vec<Elem> {
        vec![self.x, self.y, self.z]
    }
}

impl TryFromVecRef<f64> for gps::Point {
    fn try_from_vec_ref(v: &Vec<f64>) -> Result<Self, VecLenErr> {
        v.is_2d()?;
        Ok(Self { x: v[0], y: v[1] })
    }
}

impl TryFromVecRef<f64> for gps::PointZ {
    fn try_from_vec_ref(v: &Vec<f64>) -> Result<Self, VecLenErr> {
        v.is_3d()?;
        Ok(Self {
            x: v[0],
            y: v[1],
            z: v[2],
        })
    }
}

impl TryFromVecRef<Elem> for R {
    fn try_from_vec_ref(v: &Vec<Elem>) -> Result<Self, VecLenErr> {
        v.is_2d()?;
        Ok(coord! {x:v[0],y:v[1]})
    }
}

impl JsonConv<Elem, gps::Point> for J2 {}
impl JsonConv<Elem, gps::PointZ> for J3 {}

impl DieselConv2D<J2, R, Elem> for gps::Point {}
impl DieselConv3D<J3, R, Elem> for gps::PointZ {}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_diesel_2d() -> gps::Point {
        gps::Point { x: 1., y: -2. }
    }

    fn mock_diesel_3d() -> gps::PointZ {
        gps::PointZ {
            x: 1.,
            y: -2.,
            z: 3.,
        }
    }

    fn mock_json_2d() -> J2 {
        J2 {
            type_: "Point".to_string(),
            bbox: Some(vec![1., -2., 1., -2.]),
            coordinates: vec![1., -2.],
        }
    }

    fn mock_invalid_json_2d() -> J2 {
        J2 {
            type_: "Point".to_string(),
            bbox: None,
            coordinates: vec![1., -2., 3.],
        }
    }

    fn mock_json_3d() -> J3 {
        J3 {
            type_: "Point".to_string(),
            bbox: Some(vec![1., -2., 3., 1., -2., 3.]),
            coordinates: vec![1., -2., 3.],
        }
    }

    fn mock_invalid_json_3d() -> J3 {
        J3 {
            type_: "Point".to_string(),
            bbox: None,
            coordinates: vec![1., -2.],
        }
    }

    #[test]
    fn to_json_2d() {
        assert_eq!(mock_diesel_2d().try_to_json().unwrap(), mock_json_2d())
    }

    #[test]
    fn to_json_3d() {
        assert_eq!(mock_diesel_3d().try_to_json().unwrap(), mock_json_3d())
    }

    #[test]
    fn to_diesel_2d() {
        assert_eq!(mock_json_2d().try_to_diesel().unwrap(), mock_diesel_2d())
    }

    #[test]
    fn to_diesel_3d() {
        assert_eq!(mock_json_3d().try_to_diesel().unwrap(), mock_diesel_3d())
    }

    #[test]
    #[should_panic]
    fn invalid_conv_2d() {
        mock_invalid_json_2d().try_to_diesel().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_conv_3d() {
        mock_invalid_json_3d().try_to_diesel().unwrap();
    }

    #[test]
    fn default_2d() {
        mock_invalid_json_2d().to_diesel_or_default();
    }

    #[test]
    fn default_3d() {
        mock_invalid_json_3d().to_diesel_or_default();
    }
}
