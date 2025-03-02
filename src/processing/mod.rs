use crate::config;
use crate::data::Context;
use crate::io::sitegen::{RasterSiteGenerator, SiteGenerator, VectorSiteGenerator};

/// Given a site source configuration, ContextGenerator will generate a sequence of Contexts to be processed.
///
/// TODO: decouple from config. Maybe create a registry for SiteGenerators (abstract factory?) and couple it with config instead. Will allow for plugin extensibility later.
struct ContextGenerator {
    site_generator: Box<dyn SiteGenerator>,
}

impl ContextGenerator {
    pub fn new(site_src_config: config::SitesSource) -> Result<Self, Box<dyn std::error::Error>> {
        let site_generator: Box<dyn SiteGenerator> = match site_src_config.clone() {
            config::SitesSource::Vector(cfg) => Box::new(VectorSiteGenerator::new(
                cfg.file.as_str(),
                cfg.site_id_key,
            )?),
            config::SitesSource::Raster(cfg) => Box::new(RasterSiteGenerator::new(
                cfg.file.as_str(),
                cfg.layer_index,
            )?),
        };

        Ok(ContextGenerator { site_generator })
    }
}

impl Iterator for ContextGenerator {
    type Item = Context;

    fn next(&mut self) -> Option<Self::Item> {
        self.site_generator.next().map(|site| Context { site })
    }
}

#[cfg(test)]
mod tests {
    use crate::config;
    use crate::processing::ContextGenerator;

    fn generic_test(config: config::SitesSource, expected_sites: &[i32]) {
        let generator = ContextGenerator::new(config).unwrap();
        let mut min = i32::max_value();
        let mut max = i32::min_value();

        for (i, ctx) in generator.enumerate() {
            if i < expected_sites.len() {
                assert_eq!(ctx.site.id, expected_sites[i])
            }

            min = min.min(ctx.site.id);
            max = max.max(ctx.site.id);
        }

        assert_eq!(min, 3894630);
        assert_eq!(max, 4041539);
    }

    #[test]
    fn test_context_generator_vector() {
        let config = config::SitesSource::Vector(config::VectorSitesSourceConfig {
            file: "testdata/DSSAT-Soils.shp.zip".to_string(),
            site_id_key: "CELL5M".to_string(),
        });

        let expected_sites = vec![
            3989689, 3989690, 3989691, 3989692, 3989693, 3994009, 3994010, 3994011, 3994012,
            3994013, 3998329, 3998330, 3998331, 3998332, 3998333, 3998334, 4002650, 4002651,
            4002652, 4002653, 4002654, 4006970, 4006971, 4006972, 4006973, 4006974, 4006975,
            4011290, 4011291, 4011292, 4011293, 4011294, 4011295, 4011296, 4011297, 4015610,
            4015611, 4015612, 4015613, 4015614, 4015615, 4015616, 4015617, 4019930, 4019931,
            4019932, 4019933, 4019934, 4019935, 4019936, 4019937, 4024251, 4024252, 4024253,
            4024254, 4024255, 4024256, 4024257, 4024258, 4028574, 4028575, 4028576, 4028577,
            4028578, 4032895, 4032896, 4032897, 4032898, 4037216, 4037217, 4037218, 4041536,
            4041537, 4041538, 4041539, 3894630, 3898947, 3898948, 3898949, 3903264, 3903265,
            3903266, 3903267, 3903268, 3903269, 3903271, 3903273, 3903274, 3903279, 3903280,
        ];

        generic_test(config, &expected_sites);
    }

    #[test]
    fn test_context_generator_raster() {
        let config = config::SitesSource::Raster(config::RasterSitesSourceConfig {
            file: "testdata/DSSAT-Soils.tif".to_string(),
            layer_index: 0,
        });

        let expected_sites = vec![
            3894630, 3898947, 3898948, 3898949, 3898975, 3898976, 3903264, 3903265, 3903266,
            3903267, 3903268, 3903269, 3903271, 3903273, 3903274, 3903279, 3903280, 3903284,
            3903286, 3903293, 3903294, 3903295, 3903296, 3903297, 3903298, 3903299, 3907584,
            3907585, 3907586, 3907587, 3907588, 3907589, 3907591, 3907592, 3907593, 3907594,
            3907595, 3907598, 3907599, 3907602, 3907604, 3907605, 3907606, 3907607, 3907608,
            3907609, 3907610, 3907611, 3907612, 3907613, 3907614, 3907615, 3907616, 3907617,
            3907618, 3907619, 3911904, 3911905, 3911906, 3911907, 3911909, 3911910, 3911912,
            3911913, 3911914, 3911915, 3911917, 3911918, 3911919, 3911923, 3911924, 3911925,
            3911926, 3911927, 3911928, 3911929, 3911930, 3911931, 3911932, 3911933, 3911934,
            3911935, 3911936, 3911937, 3911938, 3911939, 3916224, 3916225, 3916226, 3916227,
        ];

        generic_test(config, &expected_sites);
    }
}
