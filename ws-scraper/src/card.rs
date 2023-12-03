use self::{scrap_processor::process_detail_section, types::WSCard};
use scraper::{Html, Selector};
use std::error::Error;

mod db_helper;
mod scrap_processor;
mod types;

pub async fn get_card_details(url: &str) -> Result<WSCard, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("div#cardDetail.card-detail table.card-detail-table").unwrap();
    let card_details = document.select(&selector).next().unwrap();

    let card_data = process_detail_section(&card_details)?;
    print!("{:?}", card_data);
    Ok(card_data)
}
