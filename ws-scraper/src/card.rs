use headless_chrome::Browser;
use std::error::Error;

use self::scrap_processor::process_detail_section;

mod scrap_processor;
mod types;

pub fn scrap_card_details(browser: &Browser, url: &str) -> Result<(), Box<dyn Error>> {
    let tab = browser.new_tab()?;
    let card_details = tab
        .navigate_to(url)?
        .wait_for_element("div#cardDetail.card-detail table.card-detail-table")?;

    println!("processing card details, {:?}", tab.get_url());
    let card_data = process_detail_section(&card_details)?;

    println!("card data: {:?}", card_data);

    tab.close(true)?;
    Ok(())
}
