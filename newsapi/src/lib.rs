use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    ConvertToStringFailed(std::io::Error),
    #[error("Failed article parsing")]
    ArticleParsingFailed(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApiError::ConvertToStringFailed(e))?;
    let articles: Articles =
        serde_json::from_str(&response).map_err(|e| NewsApiError::ArticleParsingFailed(e))?;
    Ok(articles)
}
