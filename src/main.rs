use colour::{dark_green, yellow};
use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::{env, error::Error};

pub fn render_articles(articles: &Articles) {
    for item in &articles.articles {
        dark_green!("> {}\n", item.title);
        yellow!("- {}\n\n", item.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // load the enviroment variables
    let news_api_key = env::var("NEWS_API_KEY").expect("NEWS_API_KEY not set");
    let mut url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=".to_owned();
    url.push_str(&news_api_key);

    let articles = get_articles(&url)?;
    render_articles(&articles);

    Ok(())
}
