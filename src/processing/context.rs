use crate::config;
use crate::data::{Context, Site};
use crate::sites::SiteGenerator;

/// Given a site source configuration, ContextGenerator will generate a sequence of Contexts to be processed.
///
/// The order of the generated Contexts is determined by a permutation over the runs and the sites iterator,
/// prioritizing outputting all the runs before moving to the next site.
///
/// TODO: decouple from config. Maybe create a registry for SiteGenerators (abstract factory?) and couple it with config instead. Will allow for plugin extensibility later.
pub struct ContextGenerator {
    site_generator: Box<dyn SiteGenerator>,
    curr_site: Option<Site>,

    runs: Vec<config::runs::RunConfig>,
    current_run: usize,
}

impl ContextGenerator {
    /// Creates a new ContextGenerator from a SitesSource configuration and a vector of RunConfig.
    #[allow(dead_code)] // The functionality required by this haven't made its way into the entrypoint yet, but this fn definitely isn't dead code.
    pub fn new(
        site_generator: Box<dyn SiteGenerator>,
        runs: Vec<config::runs::RunConfig>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ContextGenerator {
            site_generator,
            curr_site: None,
            runs,
            current_run: 0,
        })
    }
}

impl Iterator for ContextGenerator {
    type Item = Context;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_run >= self.runs.len() {
            self.current_run = 0;
            self.curr_site = None;
        }

        if self.curr_site.is_none() {
            self.curr_site = self.site_generator.next();
            self.curr_site.as_ref()?;
        }

        let run = self.runs[self.current_run].clone();
        self.current_run += 1;
        Some(Context {
            site: self.curr_site.clone()?,
            run,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::sites::gen::{RasterSiteGenerator, VectorSiteGenerator};
    use crate::sites::SiteGenerator;
    use std::collections::HashMap;

    // TODO mock a site generator. We are not testing site generators here, so there is little point in using real ones.
    fn generic_test(site_src: Box<dyn SiteGenerator>, expected_sites: &[i32]) {
        let runs = vec![
            config::runs::RunConfig {
                name: String::from("r1"),
                extra: HashMap::new(),
            },
            config::runs::RunConfig {
                name: String::from("r2"),
                extra: HashMap::new(),
            },
        ];

        let generator = ContextGenerator::new(site_src, runs).unwrap();
        let mut min = i32::max_value();
        let mut max = i32::min_value();

        for (i, ctx) in generator.enumerate() {
            let site_idx = i / 2;
            if site_idx < expected_sites.len() {
                assert_eq!(ctx.site.id, expected_sites[site_idx])
            }

            if i % 2 == 0 {
                assert_eq!(ctx.run.name, "r1");
            } else {
                assert_eq!(ctx.run.name, "r2");
            }

            min = min.min(ctx.site.id);
            max = max.max(ctx.site.id);
        }

        assert_eq!(min, 3894630);
        assert_eq!(max, 4041539);
    }

    #[test]
    fn test_context_generator_vector() {
        let site_src =
            VectorSiteGenerator::new("testdata/DSSAT-Soils.shp.zip", "CELL5M".to_string()).unwrap();

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

        generic_test(Box::new(site_src), &expected_sites);
    }

    #[test]
    fn test_context_generator_raster() {
        let site_src = RasterSiteGenerator::new("testdata/DSSAT-Soils.tif", 0).unwrap();

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

        generic_test(Box::new(site_src), &expected_sites);
    }
}
