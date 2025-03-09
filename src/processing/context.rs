use super::PipelineData;
use crate::config;
use crate::sites::Site;
use crate::sites::SiteGenerator;
use std::path::PathBuf;

/// Given a site source configuration, ContextGenerator will generate a sequence of Contexts to be processed.
///
/// The order of the generated Contexts is determined by a permutation over the runs and the sites iterator,
/// prioritizing outputting all the runs before moving to the next site.
///
/// TODO: decouple from config. Maybe create a registry for SiteGenerators (abstract factory?) and couple it with config instead. Will allow for plugin extensibility later.
pub struct ContextGenerator {
    site_generator: Box<dyn SiteGenerator>,
    curr_site: Option<Site>,
    site_sample_size: Option<usize>,
    current_site_count: usize,
    runs: Vec<config::runs::RunConfig>,
    current_run: usize,
}

impl ContextGenerator {
    /// Creates a new ContextGenerator from a SitesSource configuration and a vector of RunConfig.
    pub fn new(
        site_generator: Box<dyn SiteGenerator>,
        runs: Vec<config::runs::RunConfig>,
        site_sample_size: Option<usize>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ContextGenerator {
            site_generator,
            curr_site: None,
            site_sample_size,
            current_site_count: 0,
            runs,
            current_run: 0,
        })
    }
}

impl Iterator for ContextGenerator {
    type Item = Context;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample_size) = self.site_sample_size {
            if self.current_site_count >= sample_size {
                return None;
            }
        }

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
        self.current_site_count += 1;
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
    use crate::data::GeoDeg;
    use crate::sites::SiteGenerator;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn context_gen() {
        let site_src: Box<dyn SiteGenerator> = Box::new((0..200).map(|id| Site {
            id,
            lon: GeoDeg::from(0.0),
            lat: GeoDeg::from(0.0),
        }));

        let runs = vec![
            config::runs::RunConfig {
                name: String::from("r1"),
                extra: HashMap::new(),
                template: PathBuf::from("dummy"),
            },
            config::runs::RunConfig {
                name: String::from("r2"),
                extra: HashMap::new(),
                template: PathBuf::from("dummy"),
            },
        ];

        let generator = ContextGenerator::new(site_src, runs, None).unwrap();
        let mut max = i32::MIN;

        for (i, ctx) in generator.enumerate() {
            assert_eq!((i / 2) as i32, ctx.site.id);

            if i % 2 == 0 {
                assert_eq!(ctx.run.name, "r1");
            } else {
                assert_eq!(ctx.run.name, "r2");
            }

            max = max.max(ctx.site.id);
        }

        assert_eq!(max, 199);
    }

    #[test]
    fn test_sample_size() {
        let site_src: Box<dyn SiteGenerator> = Box::new((0..200).map(|id| Site {
            id,
            lon: GeoDeg::from(0.0),
            lat: GeoDeg::from(0.0),
        }));

        let runs = vec![config::runs::RunConfig {
            name: String::from("r1"),
            extra: HashMap::new(),
            template: PathBuf::from("dummy"),
        }];

        let generator = ContextGenerator::new(site_src, runs, Some(50)).unwrap();
        assert_eq!(generator.count(), 50);
    }

    #[test]
    fn test_context_dir() {
        let wd = PathBuf::from("/tmp");
        let ctx = Context {
            site: Site {
                id: 0,
                lon: GeoDeg::from(15.222),
                lat: GeoDeg::from(-15.23133),
            },
            run: config::runs::RunConfig {
                name: String::from("r1"),
                extra: HashMap::new(),
                template: PathBuf::from("dummy"),
            },
        };

        assert_eq!(ctx.dir(&wd), PathBuf::from("/tmp/r1/15_2220N/15_2313W"));
    }
}

/// Holds the information about the execution of a single run on a specific site with its bound run configurations.
#[derive(Debug, Clone)]
pub struct Context {
    #[allow(dead_code)]
    // The part of the code that uses this is not yet implemented, so it's not dead code.
    pub site: Site,

    #[allow(dead_code)]
    // The part of the code that uses this is not yet implemented, so it's not dead code.
    pub run: config::runs::RunConfig,
}

impl PipelineData for Context {}

impl Context {
    pub fn dir(&self, base: &PathBuf) -> PathBuf {
        let mut path = base.clone();
        path.push(&self.run.name);
        path.push(&self.site.lon.ns(4));
        path.push(&self.site.lat.ew(4));
        path
    }

    pub fn tera(&self) -> tera::Context {
        let mut ctx = tera::Context::new();
        ctx.insert("site_id", &self.site.id);
        ctx.insert("soil_id", &self.site.id); // Backwards compatibility. In the original Pythia, the site ID was the soil ID.
        ctx.insert("lng", &self.site.lon.as_f32()); // Backwards compatibility, original Pythia impl used lat/lng instead of lon/lat.
        ctx.insert("lon", &self.site.lon.as_f32());
        ctx.insert("lat", &self.site.lat.as_f32());
        ctx.insert("name", &self.run.name);

        for (k, v) in &self.run.extra {
            ctx.insert(k, v);
        }

        ctx
    }
}
