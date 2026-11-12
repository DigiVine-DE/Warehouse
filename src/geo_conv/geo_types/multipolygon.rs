use super::*;

type P<Pt> = gps::MultiPolygonC<Pt>;
type Elem = Vec<Vec<Vec<f64>>>;
type V = Vec<Elem>;
type R = geo::MultiPolygon;
type J2 = models::MultiPolygon;

impl_json_wrapper!(J2, V, "MultiPolygon");

impl<Pt: Clone + ToVec<f64>> ToVec<Elem> for P<Pt> {
    fn to_vec(&self) -> V {
        self.polygons.to_vec()
    }
}

impl<Pt> GeoCollection for P<Pt> {
    type GeoItem = gps::PolygonC<Pt>;
}

impl GeoCollection for R {
    type GeoItem = geo::Polygon;
}

impl DieselConv2D<J2, R, Elem> for P<gps::Point> {}
impl JsonConv<Elem, P<gps::Point>> for J2 {}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_vec() -> V {
        vec![
            vec![vec![vec![0., 0.], vec![5., 0.], vec![7., 9.], vec![0., 0.]]],
            vec![vec![
                vec![0., 0.],
                vec![5., 0.],
                vec![7.01, 9.],
                vec![0., 0.],
            ]],
        ]
    }

    fn mock_json() -> J2 {
        J2 {
            type_: "MultiPolygon".to_string(),
            bbox: Some(vec![0., 0., 7.01, 9.]),
            coordinates: mock_vec(),
        }
    }

    fn mock_diesel() -> P<gps::Point> {
        gps::MultiPolygon::try_from_vec_ref(&mock_vec()).unwrap()
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
