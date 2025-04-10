use crate::api::{artikel, merkzettel};
use crate::infrastructure::client::Client;
use std::error::Error;
use uritemplate::UriTemplate;

pub fn run(client: &Client) -> Result<(), Box<dyn Error>> {
    let merkzettel = client
        .request(merkzettel::METHOD, merkzettel::URL)
        .send()?
        .json::<merkzettel::Response>()?;

    let merkzettel = merkzettel.artikelnummern.chunks(10);
    let merkzettel = merkzettel
        .map(|chunk| {
            let artikel = client
                .request(
                    artikel::METHOD,
                    UriTemplate::new(artikel::URI_TEMPLATE)
                        .set("artikelnummern", chunk.join(","))
                        .build(),
                )
                .send()?
                .json::<artikel::Response>()?;
            Ok::<artikel::Response, Box<dyn Error>>(artikel)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let merkzettel = merkzettel.iter().flatten().collect::<Vec<_>>();
    merkzettel.iter().for_each(|artikel| {
        println!(
            "{} - {} - {}",
            artikel
                .autor()
                .map(|it| it.name.as_str())
                .unwrap_or("Unknown Author"),
            artikel.titel,
            artikel
                .bevorzugter_preis()
                .map(|preis| format!(
                    "{:.2} {}",
                    (preis.betrag as f32 / 100f32),
                    preis.waehrung.text
                ))
                .unwrap_or("No Price".to_string())
        )
    });

    Ok(())
}
