use super::*;

type P<Pt> = gps::LineStringC<Pt>;
type Elem = Vec<f64>;
type V = Vec<Elem>;
type R = geo::LineString;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rs_from_vec() {
        let v = vec![vec![1., 2.], vec![3., 4.]];
        let r = R::try_from_vec_ref(&v).unwrap();
        let mut coords = r.coords();
        let got = coords.next().unwrap();
        assert_eq!(got, &geo::coord! {x:1.,y:2.});

        let got = coords.next().unwrap();
        assert_eq!(got, &geo::coord! {x:3.,y:4.});
    }
}
