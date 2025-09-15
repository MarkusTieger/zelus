use url::Url;

pub trait ZelusClientImpl {
    fn base_url(&self) -> &Url;
    fn client(&self) -> &reqwest::Client;
}
