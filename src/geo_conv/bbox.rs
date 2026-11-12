use crate::response::FromJson;
use diesel_ext::GeoBBox2D;

impl FromJson<&Vec<f64>> for GeoBBox2D {
    fn from_json(v: &Vec<f64>) -> Result<Self, String> {
        Self::try_from(v)
    }
}

pub trait HeightBounding {
    fn min_max(&self) -> (f64, f64);
}

impl HeightBounding for f64 {
    fn min_max(&self) -> (f64, f64) {
        (*self, *self)
    }
}

/// find min and max value for a list of HeightBoundings
impl<T: HeightBounding> HeightBounding for Vec<T> {
    fn min_max(&self) -> (f64, f64) {
        self.iter()
            .map(|x| x.min_max())
            .fold((0., 0.), |(min, max), (item_min, item_max)| {
                (
                    if item_min < min { item_min } else { min },
                    if item_max > max { item_max } else { max },
                )
            })
    }
}

pub trait BBox {
    fn bbox(&self) -> Option<Vec<f64>>;

    fn bbox_3d<H: HeightBounding>(&self, h: H) -> Option<Vec<f64>> {
        // calculate 2D bbox and convert it to 3D
        self.bbox().map(|mut bbox| {
            let (min, max) = h.min_max();
            bbox.insert(2, min);
            bbox.push(max);
            bbox
        })
    }
}

impl<R: geo::BoundingRect<f64>> BBox for R {
    fn bbox(&self) -> Option<Vec<f64>> {
        self.bounding_rect().into().map(|x| {
            let (min, max) = (x.min(), x.max());
            vec![min.x, min.y, max.x, max.y]
        })
    }
}
