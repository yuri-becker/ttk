use reqwest::Method;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt;
use std::fmt::Formatter;

pub const URI_TEMPLATE: &str = "https://www.thalia.de/api/2003/artikel/v4/{artikelnummern}";

pub const METHOD: Method = Method::GET;

pub type Response = Vec<ResponseItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseItem {
    #[serde(rename = "detailAttributes")]
    pub detail_attributes: Option<Vec<DetailAttribut>>,
    #[serde(rename = "kategoriePfade")]
    /// Categories are organized in a hierarchy and multiple paths of categories can lead to this article.
    pub kategorie_pfade: Vec<KategoriePfad>,
    pub shop: Shop,
    pub kundenprogramm: bool,
    #[serde(rename = "salesranks")]
    pub sales_ranks: Option<Vec<Salesrank>>,
    pub media: Media,
    pub meldeschluessel: Meldeschluessel,
    pub titel: String,
    pub abmessungen: Option<Abmessungen>,
    pub personen: Vec<Person>,
    #[serde(rename = "werkArtikel")]
    pub werk_artikel: Option<Vec<WerkArtikel>>,
    pub seiten: Option<String>,
    pub kurztitel: String,
    #[serde(rename = "ID")]
    pub id: Id,
    pub zusatztexte: Vec<Zusatztext>,
    pub filialabholung: bool,
    pub seo: Seo,
    pub exklusiv: bool,
    #[serde(rename = "voeDatum")]
    pub voe_datum: VoeDatum,
    pub altersempfehlung: Option<Altersempfehlung>,
    pub originaltitel: Option<String>,
    pub abbildungsvermerk: Option<String>,
    #[serde(rename = "searchIndexMandanten")]
    pub search_index_mandanten: Vec<u32>,
    pub reihe: Option<Reihe>,
    pub sprache: IndexedValue,
    pub preise: Vec<Preis>,
    pub untertitel: Option<String>,
    pub neuheit: bool,
    pub werk: Werk,
    pub badges: Option<Vec<Badge>>,
    #[serde(rename = "schlagworteQualifiziert")]
    pub schlagworte_qualifiziert: Option<Vec<Schlagwort>>,
    pub form: Form,
    pub bestand: Bestand,
    pub auflage: Option<String>,
    pub kopierschutz: Option<Kopierschutz>,
    pub produzent: Produzent,
    #[serde(rename = "jugendschutzEinteilung")]
    pub jugendschutz_einteilung: Option<IndexedValue>,
    pub serie: Serie,
    pub migration: u32,
    #[serde(rename = "wwsZusatzdaten")]
    pub wws_zusatzdaten: WssZusatzdaten,
    pub abmessung: Option<String>,
    #[serde(rename = "versandkostenTyp")]
    pub versandkosten_typ: VersandkostenTyp,
    #[serde(rename = "spracheProdukt")]
    pub sprache_produkt: String,
    #[serde(rename = "bewertungsStatistik")]
    pub bewertungs_statistik: Option<BewertungsStatistik>,
    #[serde(rename = "artikelVerknuepfungen")]
    pub artikel_verknuepfungen: Option<Vec<ArtikelVerknuepfung>>,
    pub schlagworte: Option<Vec<IndexedValue>>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

impl ResponseItem {
    pub fn autor(&self) -> Option<&Person> {
        self.personen
            .iter()
            .find(|person| person.typ.eq(&PersonTyp::Autor))
    }

    pub fn bevorzugter_preis(&self) -> Option<&Preis> {
        let preise = &self.preise;
        if preise.is_empty() {
            return None;
        }
        if preise.len() == 1 {
            return preise.first();
        }

        let eur_verkaufspreise = preise
            .iter()
            .filter(|preis| preis.waehrung.text == "EUR" && preis.typ.eq(&PreisTyp::Verkauf))
            .collect::<Vec<_>>();
        if !eur_verkaufspreise.is_empty() {
            return eur_verkaufspreise.first().copied();
        }

        let verkaufspreise = preise
            .iter()
            .filter(|preis| preis.typ.eq(&PreisTyp::Verkauf))
            .collect::<Vec<_>>();
        verkaufspreise.first().copied()
    }
}

/// Each KategoriePfad represents one possible path leading to the article.
pub type KategoriePfad = Vec<Kategorie>;

#[derive(Deserialize, Serialize, Debug)]
/// Category for items
///
/// # Examples
///
/// ```json
/// { "identNr": 2, "text": "Bücher" }
/// ```
/// ```json
/// { "identNr": 11323, "text": "Boys Love" }
/// ```
/// ```json
/// { "identNr": 5509, "fuehrend": true, "text": "Egmont " }
/// ```
///
///
pub struct Kategorie {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    fuehrend: Option<bool>,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    bilder: Vec<Bild>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bild {
    #[serde(rename = "migrationUrlTemplateFixedScaling")]
    /// Example: "https://assets.thalia.media/img/artikel/dfc0766a01a7ed6d04b4341607e8a84b1a8bc305-00-{resolutionKey}.jpeg",
    migration_url_template_fixed_scaling: Option<String>,
    breite: Option<u32>,
    #[serde(rename = "migrationUrlTemplateCustomScaling")]
    migration_url_template_custom_scaling: Option<String>,
    hoehe: u32,
    #[serde(rename = "urlTemplateCustomScaling")]
    /// Example: "https://images.thalia.media/-/{customScaling}/c3576ffc4e1448ccb3afccc568b4ebc9/the-three-body-problem-boxset-taschenbuch-cixin-liu-englisch.jpeg"
    url_template_custom_scaling: String,
    typ: BildTyp,
    #[serde(rename = "urlTemplateFixedScaling")]
    /// Example: "https://images.thalia.media/{resolutionKey}/-/c3576ffc4e1448ccb3afccc568b4ebc9/the-three-body-problem-boxset-taschenbuch-cixin-liu-englisch.jpeg"
    url_template_fixed_scaling: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BildTyp {
    #[serde(rename = "coverbild")]
    Coverbild,
    #[serde(rename = "coverback")]
    Coverback,
    #[serde(rename = "detail")]
    Detail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meldeschluessel {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    nur_stationaer: bool,
    nachbestellbar: bool,
    lieferzeit: u32,
    text: String,
    kaufbar: MeldeschluesselKaufbar,
    lieferant: Lieferant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lieferant {
    lifnr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MeldeschluesselKaufbar {
    K,
    V,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Abmessungen {
    breite: Option<Abmessung>,
    hoehe: Option<Abmessung>,
    gewicht: Option<Abmessung>,
    laenge: Option<Abmessung>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Abmessung {
    einheit: Einheit,
    wert: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Einheit {
    #[serde(rename = "cm")]
    Centimeters,
    #[serde(rename = "g")]
    Grams,
}

#[derive(Serialize, Deserialize, Debug)]
///
/// # Examples
/// ```json
/// { "wert": [ "Given" ], "name": "Originaltitel" }
/// ```
/// ```json
/// { "wert": [ "1" ], "name": "Auflage" }
/// ```
/// ```json
/// { "wert": [ "07.05.2024" ], "name": "Erscheinungsdatum" }
/// ```
/// ```json
/// { "wert": [ "Planetes Perfect Edition 1" ], "name": "Reihe" } ```
pub struct DetailAttribut {
    name: DetailAttributName,
    /// wert seems to be – most of the time – an Array with one string item.
    /// These can sometimes be actual strings, but also numbers or German dates (see examples above).
    wert: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DetailAttributName {
    Verkaufsrang,
    Einband,
    Altersempfehlung,
    Erscheinungsdatum,
    Verlag,
    Seitenzahl,
    #[serde(rename = "Maße (L/B/H)")]
    MasseLBH,
    #[serde(rename = "Maße (L/B)")]
    MasseLB,
    Gewicht,
    Auflage,
    Originaltitel,
    #[serde(rename = "Übersetzt von")]
    UebersetztVon,
    Sprache,
    #[serde(rename = "ISBN")]
    Isbn,
    Abbildungen,
    #[serde(rename = "Hrsg. von")]
    HerausgegebenVon,
    Reihe,
    #[serde(rename = "Illustriert von")]
    IllustriertVon,
    #[serde(rename = "Gesprochen von")]
    GesprochenVon,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
/// No idea what this represents.
pub struct Shop {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    kurzbezeichnung: String,
    name: String,
    label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Salesrank {
    #[serde(rename = "rankCount")]
    rank_count: u32,
    #[serde(rename = "shopId")]
    shop_id: OptionalIdentNr,
    category: OptionalIdentNr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    #[serde(rename = "identNr")]
    pub ident_nr: IdentNr,
    pub name: String,
    pub typ: PersonTyp,
    pub reihenfolge: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum PersonTyp {
    Autor,
    #[serde(rename = "Übersetzer")]
    Uebersetzer,
    Herausgeber,
    Illustrator,
    Sprecher,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    werks: String,
    ean: String,
    #[serde(rename = "mergeDiscriminator")]
    merge_discriminator: MergeDiscriminator,
    isbn13: Option<String>,
    matnr: String,
    #[serde(rename = "mandantId")]
    mandant_id: u32,
    isbn10: Option<String>,

    alle_ean: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MergeDiscriminator {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "EPUB")]
    Epub,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Zusatztext {
    typ: IndexedValue,
    /// HTML formatted
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Seo {
    #[serde(rename = "searchIndexMandanten")]
    search_index_mandanten: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoeDatum {
    jahr: u16,
    tag: u8,
    monat: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Altersempfehlung {
    von: u8,
    bis: Option<u8>,
    alterseinheit: Alterseinheit,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Alterseinheit {
    J,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preis {
    pub streichpreisart: Option<Streichpreisart>,
    #[serde(rename = "rabattProzent")]
    pub rabatt_prozent: Option<f32>,
    #[serde(rename = "anzeigerabattProzent")]
    pub anzeigerabatt_prozent: Option<u32>,
    pub betrag: u32,
    pub waehrung: IndexedValue,
    pub typ: PreisTyp,
    #[serde(rename = "preisnachlassBetrag")]
    pub preisnachlass_betrag: Option<u32>,
    pub preisbindung: Option<bool>,
    pub grundpreispflichtig: Option<bool>,
    pub mehrwertsteuer: Mehrwertsteuer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mehrwertsteuer {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    #[serde(rename = "mwstSatz")]
    mwst_satz: u32,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum PreisTyp {
    #[serde(rename = "verkauf")]
    Verkauf,
    #[serde(rename = "streich")]
    Streich,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Streichpreisart {
    #[serde(rename = "UVP")]
    Uvp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Werk {
    #[serde(rename = "alleinstellungsMerkmal")]
    alleinstellungs_merkmal: String,
    #[serde(rename = "gruppierungsMerkmal")]
    gruppierungs_merkmal: GruppierungsMerkmal,
    #[serde(rename = "wzwAvailable")]
    wzw_available: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GruppierungsMerkmal {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    text: String,
    #[serde(rename = "alleinstellungsMerkmalLabel")]
    alleinstellungs_merkmal_label: String,
    #[serde(rename = "ordnungsNummer")]
    ordnungs_nummer: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Badge {
    farbe_designsystem: String,
    badge: String,
    text: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    navigation: String,
    typ: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bestand {
    #[serde(rename = "bestandsIndikator")]
    bestands_indikator: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kopierschutz {
    text: String,
    kopiergeschuetzt: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Produzent {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    name: String,
    typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Serie {
    #[serde(rename = "hatSerienslider")]
    hat_serienslider: bool,
    name: Option<String>,
    nummer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WssZusatzdaten {
    aufmachung: String,
    sprachindex: String,
    medienindex: String,
    #[serde(rename = "warengruppeSbs")]
    warengruppe_sbs: WarengruppeSbs,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WarengruppeSbs {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VersandkostenTyp {
    Versandkostenfrei,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BewertungsStatistik {
    total: u32,
    #[serde(rename = "sterneStatistik")]
    sterne_statistik: Vec<SterneStatistik>,
    durchschnitt: f32,
    #[serde(rename = "anzahlSterne")]
    anzahl_sterne: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SterneStatistik {
    total: u32,
    relativ: u32,
    #[serde(rename = "anzahlSterne")]
    anzahl_sterne: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schlagwort {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WerkArtikel {
    #[serde(rename = "voeDatum")]
    voe_datum: VoeDatum,
    shop: WerkArtikelShop,
    media: Media,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WerkArtikelShop {
    #[serde(rename = "identNr")]
    ident_nr: IdentNr,
    label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtikelVerknuepfung {
    typ: IndexedValue,
    ziel: ArtikelVerknuepfungZiel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtikelVerknuepfungZiel {
    ean: String,
    isbn: Option<String>,
    matnr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reihe {
    name: String,
    nummer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexedValue {
    #[serde(rename = "identNr")]
    pub ident_nr: Option<IdentNr>,
    pub text: String,
}

pub type IdentNr = u32;

#[derive(Debug)]
pub enum OptionalIdentNr {
    None,
    Some(IdentNr),
}

impl<'de> Deserialize<'de> for OptionalIdentNr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OptionalIdentNrVisitor;

        impl Visitor<'_> for OptionalIdentNrVisitor {
            type Value = OptionalIdentNr;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str("-1 or a positive integer")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v > 0 {
                    Ok(OptionalIdentNr::Some(v as u32))
                } else {
                    Err(E::custom("Value most not be 0"))
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v < 0 {
                    Ok(OptionalIdentNr::Some(v as u32))
                } else if v == -1 {
                    Ok(OptionalIdentNr::None)
                } else {
                    Err(E::custom("Value is not -1 or a positive integer"))
                }
            }
        }
        deserializer.deserialize_i64(OptionalIdentNrVisitor)
    }
}

impl Serialize for OptionalIdentNr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            OptionalIdentNr::None => -1,
            OptionalIdentNr::Some(ident_nr) => *ident_nr as i32,
        })
    }
}
