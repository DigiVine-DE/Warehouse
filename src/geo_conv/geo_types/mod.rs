macro_rules! impl_json_wrapper {
    ($json:ident, $vec:ident, $name:expr) => {
        impl Wrapper for $json {
            type Inner = $vec;

            fn as_inner(&self) -> &Self::Inner {
                &self.coordinates
            }

            fn from_inner(inner: Self::Inner) -> Self {
                Self {
                    type_: $name.to_string(),
                    bbox: None,
                    coordinates: inner,
                }
            }
        }

        impl JsonWrapper for $json {
            fn update_bbox(&mut self, bbox: Option<Vec<f64>>) {
                self.bbox = bbox
            }
        }
    };
}

mod linestring;
mod multipoint;
mod multipolygon;
mod point;
mod polygon;

use super::{
    conv::{DieselConv2D, DieselConv3D, JsonConv},
    error::{LengthChecker, PointVecChecker, VecLenErr},
    vec_traits::{GeoCollection, ToVec, TryFromVecRef},
    wrapper::{JsonWrapper, Wrapper},
};
use digivine_server::models;
use postgis_diesel::gps;
