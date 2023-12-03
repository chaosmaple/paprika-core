use std::error::Error;

pub mod card;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    card::get_card_details("https://ws-tcg.com/cardlist/?cardno=ID/W13-124&l").await?;
    Ok(())
}
