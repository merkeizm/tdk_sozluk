use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use hyper::body::to_bytes;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Anlam {
    pub anlam: String,
    #[serde(default)]
    pub orneklerListe: Option<Vec<Ornek>>,
    #[serde(default)]
    pub ozelliklerListe: Option<Vec<Ozellik>>,
}

#[derive(Debug, Deserialize)]
pub struct Ornek {
    pub ornek: String,
}

#[derive(Debug, Deserialize)]
pub struct Ozellik {
    pub tam_adi: String,
}

#[derive(Debug, Deserialize)]
pub struct KelimeVeri {
    pub madde: String,
    pub anlamlarListe: Vec<Anlam>,
    #[serde(default)]
    pub birlesikler: Option<String>,
    #[serde(default)]
    pub atasozu: Option<Vec<Atasozu>>,
    #[serde(default)]
    pub telaffuz: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Atasozu {
    pub madde: String,
}

pub struct KelimeDetay {
    pub kelime: String,
    pub anlamlar: Vec<(String, Vec<String>, Vec<String>, Option<String>)>,
    pub birlesik_kelimeler: Vec<String>,
    pub atasozleri: Vec<String>,
    pub isaret_dili_gifleri: Vec<String>,
    pub sesli_okunus_url: Option<String>,
}

pub async fn kelime(kelime: &str) -> Option<KelimeDetay> {
    let url = format!("https://sozluk.gov.tr/gts?ara={}", kelime);
    let uri: Uri = url.parse().ok()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(uri).await.ok()?;
    let body_bytes = to_bytes(res).await.ok()?;

    let veri: Vec<KelimeVeri> = serde_json::from_slice(&body_bytes).ok()?;

    let ilk = veri.first()?;

    let anlamlar = ilk.anlamlarListe.iter().map(|a| {
        let ornekler = a.orneklerListe
            .as_ref()
            .map(|o| o.iter().map(|o| o.ornek.clone()).collect())
            .unwrap_or_else(Vec::new);

        let ozellikler = a.ozelliklerListe
            .as_ref()
            .map(|o| o.iter().map(|o| o.tam_adi.clone()).collect())
            .unwrap_or_else(Vec::new);

        (a.anlam.clone(), ornekler, ozellikler, ilk.telaffuz.clone())
    }).collect();

    let birlesik_kelimeler = ilk.birlesikler
        .as_ref()
        .map(|b| b.split(", ").map(String::from).collect())
        .unwrap_or_else(Vec::new);

    let atasozleri = ilk.atasozu
        .as_ref()
        .map(|a| a.iter().map(|x| x.madde.clone()).collect())
        .unwrap_or_else(Vec::new);

    let isaret_dili_gifleri = kelime.chars()
        .map(|c| format!("https://sozluk.gov.tr/assets/img/isaret/{}.gif", c))
        .collect();

    let sesli_okunus_url = kelime_sesli_okunus(kelime).await;

    Some(KelimeDetay {
        kelime: ilk.madde.clone(),
        anlamlar,
        birlesik_kelimeler,
        atasozleri,
        isaret_dili_gifleri,
        sesli_okunus_url,
    })
}

pub async fn kelime_sesli_okunus(kelime: &str) -> Option<String> {
    let url = format!("https://sozluk.gov.tr/yazim?ara={}", kelime);
    let uri: Uri = url.parse().ok()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(uri).await.ok()?;
    let body_bytes = to_bytes(res).await.ok()?;
    let veri: serde_json::Value = serde_json::from_slice(&body_bytes).ok()?;

    let ses_kod = veri.get(0)?.get("seskod")?.as_str()?;
    Some(format!("https://sozluk.gov.tr/ses/{}.wav", ses_kod))
}
