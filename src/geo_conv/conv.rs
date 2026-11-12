use super::{
    bbox::{BBox, HeightBounding},
    error::VecLenErr,
    vec_traits::{GeoDefaultVec, ToVec, TryFromVecRef},
    with_height::To2D,
    wrapper::JsonWrapper,
};

/// A Trait to convert diesel geo type to Json and Rust geo type.
/// if Self can be converted to a vector, and this vector can be converted to json and rust geo.
/// Rust Geo provides a bunch of algorithms to help converting diesel to json, such as calculating bounding rect.
pub trait DieselConv2D<Json, RsGeo, Elem>: ToVec<Elem>
where
    Json: JsonWrapper<Inner = Vec<Elem>>,
    RsGeo: TryFromVecRef<Elem> + geo::BoundingRect<f64>,
    Elem: GeoDefaultVec,
{
    fn try_to_rs(&self) -> Result<RsGeo, VecLenErr> {
        RsGeo::try_from_vec_ref(&self.to_vec())
    }

    fn try_to_json(&self) -> Result<Json, VecLenErr> {
        let v = self.to_vec();
        let bbox = RsGeo::try_from_vec_ref(&v)?.bbox();
        let mut json = Json::from_inner(v);
        json.update_bbox(bbox);
        Ok(json)
    }

    fn to_json_or_default(&self) -> Json {
        match self.try_to_json() {
            Ok(j) => j,
            Err(e) => {
                error!("could not convert diesel 2D to JSON: {}", e);
                Json::from_inner(Elem::geo_default_vec())
            }
        }
    }
}

/// A Trait to convert 3D diesel geo type to Json geo type.
/// The 3D diesel type will first be converted to 2D, so that the bounding rect can be calculated by Rust Geo, then it will be converted back to 3D
pub trait DieselConv3D<Json, RsGeo, Elem>: ToVec<Elem>
where
    Json: JsonWrapper<Inner = Vec<Elem>>,
    RsGeo: TryFromVecRef<Elem> + geo::BoundingRect<f64>,
    Elem: GeoDefaultVec,
    Vec<Elem>: To2D + Clone,
    <Vec<Elem> as To2D>::Height: HeightBounding,
{
    fn try_to_rs(&self) -> Result<RsGeo, VecLenErr> {
        RsGeo::try_from_vec_ref(&self.to_vec())
    }

    fn try_to_json(&self) -> Result<Json, VecLenErr> {
        let vec_3d = self.to_vec();
        let (vec_2d, height) = vec_3d.clone().to_2d()?;

        let rs = RsGeo::try_from_vec_ref(&vec_2d)?;
        let bbox = rs.bbox_3d(height);
        let mut json = Json::from_inner(vec_3d);
        json.update_bbox(bbox);
        Ok(json)
    }

    fn to_json_or_default(&self) -> Json {
        match self.try_to_json() {
            Ok(j) => j,
            Err(e) => {
                error!("could not convert diesel 3D to JSON: {}", e);
                Json::from_inner(Elem::geo_default_vec_3d())
            }
        }
    }
}

/// A Trait to convert JSON Geo type to diesel Geo type.
/// The `postgis` type must implement `EwkbRead` so that we can know whether it is a 2D or 3D geo type.
/// The converted Diesel Geo struct is guaranteed to be valid, meaning it will not cause a runtime panic.
/// However, it does not ensure that data can be successfully inserted into the database, e.g., a polygon may contain non-closed rings.
pub trait JsonConv<Elem, Diesel>: JsonWrapper<Inner = Vec<Elem>>
where
    Diesel: TryFromVecRef<Elem> + Default,
    Elem: GeoDefaultVec,
{
    fn try_to_diesel_option(&self) -> Result<Option<Diesel>, VecLenErr> {
        if self.as_inner().len() == 0 {
            Ok(None)
        } else {
            let d = Diesel::try_from_vec_ref(self.as_inner())?;
            Ok(Some(d))
        }
    }

    fn try_to_diesel(&self) -> Result<Diesel, VecLenErr> {
        self.try_to_diesel_option()?.ok_or_else(|| VecLenErr {
            got_len: self.as_inner().len(),
            msg: "Geo object here must have non-empty coordinates".into(),
        })
    }

    fn to_diesel_or_default(&self) -> Diesel {
        match self.try_to_diesel() {
            Ok(d) => d,
            Err(e) => {
                error!("could not convert json to diesel: {}", e);
                Diesel::default()
            }
        }
    }
}
