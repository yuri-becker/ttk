use crate::api::{artikel, merkzettel};
use crate::export;
use crate::infrastructure::client::Client;
use crate::infrastructure::request_cache::RequestCache;
use crate::infrastructure::templates::Templates;
use anyhow::{Context, Result};
use std::fs::write;
use uritemplate::UriTemplate;

static ARTIKEL_BATCH_SIZE: usize = 50;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[arg(
        long = "output",
        short = 'o',
        required = true,
        value_name = "file",
        help = "File the Merkzettel should be written to (file is created/overwritten)."
    )]
    output: String,

    #[arg(
        long = "cached",
        short = 'c',
        default_value = "false",
        help = "Caches Thalia responses, useful for development since otherwise tokens may be invalidated for sussy behaviour."
    )]
    cached: bool,

    #[arg(
        long = "no-google-fonts",
        default_value = "false",
        help = "Removes the Google Fonts import."
    )]
    no_google_fonts: bool
}

impl Args {
    pub fn run(self, client: &Client, templates: &Templates, cache: &RequestCache) -> Result<()> {
        let merkzettel = cache.get_cached::<merkzettel::Response>(
            self.cached,
            merkzettel::METHOD.as_str(),
            merkzettel::URL,
            || {
                client
                    .request(merkzettel::METHOD, merkzettel::URL)
                    .send()?
                    .json::<merkzettel::Response>()
                    .with_context(|| {
                        format!(
                            "Could not parse response for {} {}",
                            merkzettel::METHOD,
                            merkzettel::URL
                        )
                    })
            },
        )?;

        let merkzettel = merkzettel.artikelnummern.chunks(ARTIKEL_BATCH_SIZE);
        let merkzettel = merkzettel
            .map(|chunk| {
                let url = UriTemplate::new(artikel::URI_TEMPLATE)
                    .set("artikelnummern", chunk.join(","))
                    .build();
                cache.get_cached::<artikel::Response>(
                    self.cached,
                    artikel::METHOD.as_str(),
                    &url,
                    || {
                        client
                            .request(artikel::METHOD, &url)
                            .send()?
                            .json::<artikel::Response>()
                            .with_context(|| {
                                format!("Could not parse response for {} {}", artikel::METHOD, url)
                            })
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let merkzettel = export::merkzettel::Merkzettel {
            no_google_fonts: self.no_google_fonts,
            items: merkzettel
                .into_iter()
                .flatten()
                .map(|it| export::merkzettel::Item {
                    titel: it.titel.clone(),
                    autor: it.autor().map(|it| it.name.clone()),
                    beschreibung: it.beschreibung(),
                    preis: it
                        .bevorzugter_preis()
                        .map(|it| format!("{:.2} {}", it.betrag as f32 / 100f32, it.waehrung.text)),
                    cover: it.cover().map(|it| it.url()),
                    meldeschluessel: it.meldeschluessel.text.clone(),
                    link: it.link(),
                    isbn: it.isbn()
                })
                .collect(),
        }
            .render(templates)
            .with_context(|| "Could not render Merkzettel.")?;

        write(&self.output, merkzettel)
            .with_context(|| format!("Could not write into output file \"{}\".", &self.output))?;

        Ok(())
    }
}
