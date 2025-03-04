use crate::registry::Resource;
use crate::sites::SiteGenerator;
use crate::sites::SiteGeneratorDriver;
use std::any::Any;

pub struct SiteGeneratorDriverResource(
    pub SiteGeneratorDriver<Box<dyn SiteGenerator>, Box<dyn Any>>,
);

impl Resource for SiteGeneratorDriverResource {}
