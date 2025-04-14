use anyhow::Result;
use handlebars::Handlebars;
use std::ops::Deref;

pub struct Templates<'a> {
    instance: Handlebars<'a>,
}

impl Templates<'_> {
    pub fn new() -> Result<Self> {
        let mut instance = Handlebars::new();
        instance.register_template_string(
            "merkzettel",
            include_str!("../export/merkzettel.hbs"),
        )?;
        Ok(Self { instance })
    }
}

impl<'a> Deref for Templates<'a> {
    type Target = Handlebars<'a>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.instance
    }
}
