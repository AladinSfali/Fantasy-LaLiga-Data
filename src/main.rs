use futures::future::join_all;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

/// Fetches the HTML content of a given URL.
///
/// # Arguments
///
/// * `url` - The URL to fetch.
///
/// # Returns
///
/// A `Result` containing the HTML content as a `String` or a `reqwest::Error`.
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

/// Parses the HTML to extract player names from anchor tags with the class "jugador".
///
/// # Arguments
///
/// * `html` - The HTML content to parse.
///
/// # Returns
///
/// A `Vec<String>` containing the extracted player names.
fn parse_players(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a.jugador").expect("Failed to parse selector");
    document
        .select(&selector)
        .map(|element| element.text().collect::<String>())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let team_slugs = vec![
        "alaves", "athletic", "atletico", "barcelona", "betis", "celta", "elche",
        "espanyol", "getafe", "girona", "levante", "mallorca", "osasuna",
        "rayo-vallecano", "real-madrid", "real-oviedo", "real-sociedad", "sevilla",
        "valencia", "villarreal",
    ];

    let mut tasks = Vec::new();

    println!("Starting to scrape player data for {} teams...", team_slugs.len());

    for slug in team_slugs {
        let task = tokio::spawn(async move {
            let url = format!(
                "https://www.futbolfantasy.com/laliga/equipos/{}/plantilla",
                slug
            );
            let html = fetch_url(&url).await.unwrap_or_else(|_| String::new());
            let players = parse_players(&html);
            (slug.to_uppercase(), players)
        });
        tasks.push(task);
    }

    let results = join_all(tasks).await;

    let mut file = File::create("laliga_players.txt")?;
    let mut output_content = String::new();

    for result in results {
        let (team, players) = result?;
        output_content.push_str(&format!("
{}
", team));
        output_content.push_str("--------------------
");
        for player in players {
            output_content.push_str(&format!("{}
", player));
        }
    }

    file.write_all(output_content.as_bytes())?;

    println!("Successfully scraped all teams and saved the data to laliga_players.txt");

    Ok(())
}