use super::*;

type P<Pt> = gps::PolygonC<Pt>;
type Elem = Vec<Vec<f64>>;
type V = Vec<Elem>;
type R = geo::Polygon;
type J2 = models::Polygon;

impl_json_wrapper!(J2, V, "Polygon");

impl<Pt: Clone + ToVec<f64>> ToVec<Elem> for P<Pt> {
    fn to_vec(&self) -> V {
        self.rings.to_vec()
    }
}

impl<Pt> GeoCollection for P<Pt> {
    type GeoItem = gps::LineStringC<Pt>;
}

impl TryFromVecRef<Elem> for R {
    fn try_from_vec_ref(v: &Vec<Elem>) -> Result<Self, VecLenErr> {
        v.assert_len_gteq(1, || "Polygon in Rust Geo must have at least one ring")?;
        let mut rings = v.iter().map(geo::LineString::try_from_vec_ref);
        let outer = rings.next().unwrap(); // infallible because rings have at least one element
        let inner: Result<Vec<_>, VecLenErr> = rings.collect();
        Ok(R::new(outer?, inner?))
    }
}

impl DieselConv2D<J2, R, Elem> for P<gps::Point> {}
impl JsonConv<Elem, P<gps::Point>> for J2 {}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_invalid_diesel() -> P<gps::Point> {
        P { rings: vec![] }
    }

    #[test]
    #[should_panic]
    fn invalid_json_conv() {
        mock_invalid_diesel().try_to_json().unwrap();
    }

    #[test]
    fn default() {
        println!("{:?}", mock_invalid_diesel().to_json_or_default());
        mock_invalid_diesel().to_json_or_default();
    }
}
