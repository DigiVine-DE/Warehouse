use super::*;

type P<Pt> = gps::MultiPointC<Pt>;
type Elem = Vec<f64>;
type V = Vec<Elem>;
type R = geo::MultiPoint;
type J3 = models::MultiPointWithHeight;

impl_json_wrapper!(J3, V, "MultiPoint");

impl<Pt: Clone + ToVec<f64>> ToVec<Elem> for P<Pt> {
    fn to_vec(&self) -> V {
        self.points.to_vec()
    }
}

impl<Pt> GeoCollection for P<Pt> {
    type GeoItem = Pt;
}

impl GeoCollection for R {
    type GeoItem = geo::Coord;
}

impl DieselConv3D<J3, R, Elem> for P<gps::PointZ> {}
impl JsonConv<Elem, P<gps::PointZ>> for J3 {}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_vec() -> V {
        vec![
            vec![0., 0., 0.],
            vec![5., 0., 1.],
            vec![7., 9., 1.],
            vec![0., 0., 0.],
        ]
    }

    fn mock_json() -> J3 {
        J3 {
            type_: "MultiPoint".to_string(),
            bbox: Some(vec![0., 0., 0., 7., 9., 1.]),
            coordinates: mock_vec(),
        }
    }

    fn mock_diesel() -> P<gps::PointZ> {
        P::<gps::PointZ>::try_from_vec_ref(&mock_vec()).unwrap()
    }

    #[test]
    fn to_json() {
        assert_eq!(mock_diesel().try_to_json().unwrap(), mock_json())
    }

    #[test]
    fn to_diesel() {
        assert_eq!(mock_json().try_to_diesel().unwrap(), mock_diesel())
    }
}
