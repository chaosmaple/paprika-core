use headless_chrome::Browser;
use std::error::Error;

mod card;

fn main() -> Result<(), Box<dyn Error>>{
    let browser = Browser::default()?;
    let url = get_card_url(&browser)?;
    println!("url: {}", url);
    card::scrap_card_details(&browser, "https://ws-tcg.com/cardlist/?cardno=ID/W13-124&l")?;
    Ok(())
}

fn get_card_url(browser: &Browser) -> Result<String, Box<dyn Error>> {
    let tab = browser.new_tab()?;

    tab.navigate_to("https://ws-tcg.com/cardlist/")?
        .wait_for_element("#titleNumberList")?;
    let elem = tab
        .find_elements(".expansion-list-side.w ul.expansion-list li")?;

    elem[0].click()?;

    tab.wait_for_element("div.search-result-table-container")
        ?
        .find_element("a")
        ?
        .click()?;

    tab.wait_until_navigated()?;

    let url = tab.get_url();
    tab.close(true)?; // close tab (close browser will close all tabs

    Ok(url)
}

