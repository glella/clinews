mod theme;

// use colour::{dark_green, yellow};
use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use std::{env, error::Error};

// pub fn render_articles(articles: &Articles) {
//     for item in &articles.articles {
//         dark_green!("> {}\n", item.title);
//         yellow!("- {}\n\n", item.url);
//     }
// }

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in articles {
        theme.print_text(&format!("`{}`", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

// fn main() -> Result<(), Box<dyn Error>> {
//     dotenv().ok(); // load the enviroment variables
//     let news_api_key = env::var("NEWS_API_KEY").expect("NEWS_API_KEY not set");

//     let mut news_api = NewsAPI::new(&news_api_key);
//     news_api
//         .endpoint(Endpoint::TopHeadlines)
//         .country(Country::Us);
//     let response = news_api.fetch()?;
//     let articles = response.articles();
//     render_articles(articles);

//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // load the enviroment variables
    let news_api_key = env::var("NEWS_API_KEY").expect("NEWS_API_KEY not set");

    let mut news_api = NewsAPI::new(&news_api_key);
    news_api
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::Us);
    let response = news_api.fetch_async().await?;
    let articles = response.articles();
    render_articles(articles);

    Ok(())
}
