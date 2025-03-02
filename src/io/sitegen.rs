use std::rc::Rc;
use gdal::errors::GdalError;
use gdal::{Dataset};
use gdal::vector::{Feature, LayerAccess, FeatureIterator, Layer};
use crate::data::data;

/// SiteGenerator allows for streaming Sites from an undetermined source.
/// The order of the sites is not guaranteed, as different file formats may index their data differently, and pre-sorting is not possible.
pub trait SiteGenerator: Iterator<Item=data::Site> {}
impl<T: Iterator<Item=data::Site>> SiteGenerator for T {}


/// Implementation of SiteGenerator that allows streaming from a GDAL vector dataset.
/// Example usage with https://dataverse.harvard.edu/dataset.xhtml?persistentId=doi:10.7910/DVN/1PEEY0:
/// ```rs
/// match VectorSiteGenerator::new("Point5m_SoilGrids-for-DSSAT-10km_v1.shp.zip", "CELL5M".to_string()) {
///     Ok(gen) => for site in gen {
///         println!("{:?}", site);
///     },
///     Err(e) => println!("{}", e),
/// } 
/// ```
pub struct VectorSiteGenerator {
    site_id_key: String,
    ds: Rc<Dataset>,
    curr_layer: usize,
    layer: Option<Layer<'static>>,
    feat_iter: Box<Option<FeatureIterator<'static>>>,
}

impl<'a> VectorSiteGenerator {
    
    /// Constructs a new VectorSiteGenerator from a GDAL vector dataset.
    /// Parameter "path" is the GDAL-valid path to the dataset.
    /// Parameter "site_id_key" is the name of the field in the dataset that contains the site ID. Must be an int32, otherwise the feature is skipped.
    pub fn new(path: &str, site_id_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let ds = Rc::new(Dataset::open(path)?);
        Ok(VectorSiteGenerator { site_id_key, ds, curr_layer: 0, layer: None, feat_iter: Box::new(None) })
    }
}

impl Iterator for VectorSiteGenerator {
    type Item = data::Site;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.feat_iter.is_none() {
            self.layer = self.ds.layer(self.curr_layer).ok().map(|l| unsafe { std::mem::transmute(l) });
            if let Some(layer) = self.layer.as_mut() {
                self.feat_iter = Box::new(Some(unsafe { std::mem::transmute(layer.features()) }));
                return self.next();
            }
            return None;
        }

        match self.feat_iter.as_mut() {
            Some(feat_iter) => match feat_iter.next() {
                Some(feat) => feature_to_site(&feat, &self.site_id_key),
                None => {
                    self.curr_layer += 1;
                    self.feat_iter = Box::new(None);
                    self.next()
                }
            },
            None => None,
        }
    }
}

fn feature_to_site(feature: &Feature, site_id_key: &str) -> Option<data::Site> {
    if let Some(geometry) = feature.geometry() {
        if geometry.geometry_type() != gdal::vector::OGRwkbGeometryType::wkbPoint {
            return None
        }

        // TODO better error handling. At least expose something in the interface to let consumers know if something went wrong.
        let id_result = feature.field(site_id_key)
            .and_then(|id| id.ok_or(GdalError::NullPointer {method_name: "dummy", msg: "Feature has no id".to_string()}))
            .map(|id| id.into_int())
            .and_then(|id| id.ok_or(GdalError::NullPointer {method_name: "dummy", msg: "Feature ID is not i32".to_string()}));

        if let Ok(id) = id_result {
            let (lon, lat, _) = geometry.get_point(0);
            return Some(data::Site { id, lon: data::GeoDeg::from(lon), lat: data::GeoDeg::from(lat) })
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_site_generator() {
        let gen = VectorSiteGenerator::new("testdata/DSSAT-Soils.shp.zip", "CELL5M".to_string()).unwrap();
        let i = 0;

        let expected = vec![
            data::Site { id: 3989689, lon: data::GeoDeg::from(14.125), lat: data::GeoDeg::from(13.042) },
            data::Site { id: 3989690, lon: data::GeoDeg::from(14.208), lat: data::GeoDeg::from(13.042) },
            data::Site { id: 3989691, lon: data::GeoDeg::from(14.292), lat: data::GeoDeg::from(13.042) },
            data::Site { id: 3989692, lon: data::GeoDeg::from(14.375), lat: data::GeoDeg::from(13.042) },
            data::Site { id: 3989693, lon: data::GeoDeg::from(14.458), lat: data::GeoDeg::from(13.042) },
            data::Site { id: 3994009, lon: data::GeoDeg::from(14.125), lat: data::GeoDeg::from(12.958) },
            data::Site { id: 3994010, lon: data::GeoDeg::from(14.208), lat: data::GeoDeg::from(12.958) },
            data::Site { id: 3994011, lon: data::GeoDeg::from(14.292), lat: data::GeoDeg::from(12.958) },
            data::Site { id: 3994012, lon: data::GeoDeg::from(14.375), lat: data::GeoDeg::from(12.958) },
            data::Site { id: 3994013, lon: data::GeoDeg::from(14.458), lat: data::GeoDeg::from(12.958) },
            data::Site { id: 3998329, lon: data::GeoDeg::from(14.125), lat: data::GeoDeg::from(12.875) },
            data::Site { id: 3998330, lon: data::GeoDeg::from(14.208), lat: data::GeoDeg::from(12.875) },
            data::Site { id: 3998331, lon: data::GeoDeg::from(14.292), lat: data::GeoDeg::from(12.875) },
            data::Site { id: 3998332, lon: data::GeoDeg::from(14.375), lat: data::GeoDeg::from(12.875) },
            data::Site { id: 3998333, lon: data::GeoDeg::from(14.458), lat: data::GeoDeg::from(12.875) },
            data::Site { id: 3998334, lon: data::GeoDeg::from(14.542), lat: data::GeoDeg::from(12.875) },
            data::Site { id: 4002650, lon: data::GeoDeg::from(14.208), lat: data::GeoDeg::from(12.792) },
            data::Site { id: 4002651, lon: data::GeoDeg::from(14.292), lat: data::GeoDeg::from(12.792) },
            data::Site { id: 4002652, lon: data::GeoDeg::from(14.375), lat: data::GeoDeg::from(12.792) },
            data::Site { id: 4002653, lon: data::GeoDeg::from(14.458), lat: data::GeoDeg::from(12.792) },
        ];
        
        let len = expected.len();

        let mut min_lon: f32 = 180.0;
        let mut max_lon: f32 = -180.0;
        let mut min_lat: f32 = 90.0;
        let mut max_lat: f32 = -90.0;

        let mut i = 0;
        for site in gen {
            if i < len {
                assert_eq!(site, expected[i]);
            }

            min_lon = min_lon.min(site.lon.as_f32());
            max_lon = max_lon.max(site.lon.as_f32());
            min_lat = min_lat.min(site.lat.as_f32());
            max_lat = max_lat.max(site.lat.as_f32());
            i += 1;
        }
        
        assert_eq!(i, 1157);
        assert_eq!(min_lon, 12.042);
        assert_eq!(max_lon, 14.958);
        assert_eq!(min_lat, 12.042);
        assert_eq!(max_lat, 14.875);
    }
}