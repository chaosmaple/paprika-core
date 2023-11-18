use headless_chrome::Element;
use std::error::Error;

use super::types::*;

pub(crate) fn process_detail_section(detail_section: &Element) -> Result<WSCard, Box<dyn Error>> {
    let card_details = detail_section.find_elements("tr")?;
    let mut card = WSCard::default();

    for tr in card_details {
        if let Some(class) = tr.get_attribute_value("class")? {
            if class == "first" {
                process_picture_n_names(&tr, &mut card)?;
            } else {
                unreachable!("unrecognized class: {:?}", class);
            }
        } else {
            process_row(&tr, &mut card)?;
        }
    }
    Ok(card)
}

fn process_picture_n_names(tr: &Element, card: &mut WSCard) -> Result<(), Box<dyn Error>> {
    let tds = tr.find_elements("td")?;
    let image = tds[0].find_element("img")?.get_attribute_value("src")?;
    let card_name = tds[1].get_inner_text()?;
    let mut card_names = card_name.split("\n");
    card.image = image.unwrap().to_string();
    card.card_name = card_names.next().unwrap().to_string();
    card.card_name_kana = card_names.next().unwrap().to_string();
    Ok(())
}

fn process_row(elem: &Element, card: &mut WSCard) -> Result<(), Box<dyn Error>> {
    let tds = elem.find_elements("th,td")?;
    for pair in tds.chunks(2) {
        let column_name = pair[0].get_inner_text()?;
        let inner_text = pair[1].get_inner_text()?;
        println!("processing {}: {}", column_name, inner_text);
        match column_name.as_str() {
            "カード番号" => card.card_no = parse_text(&inner_text),
            "商品名" => card.product = parse_text(&inner_text),
            "ネオスタンダード区分" => card.expansion = parse_text(&inner_text),
            "作品番号" => card.expansion_id = parse_text(&inner_text),
            "レアリティ" => card.rarity = parse_text(&inner_text),
            "サイド" => card.side = parse_side(&pair[1])?,
            "種類" => card.card_type = parse_card_type(&inner_text)?,
            "色" => card.color = parse_color(&pair[1])?,
            "レベル" => card.level = parse_number(&inner_text)?,
            "コスト" => card.cost = parse_number(&inner_text)?,
            "パワー" => card.power = parse_number(&inner_text)?,
            "ソウル" => card.soul = parse_soul(&pair[1])? as u16,
            "トリガー" => card.trigger = parse_trigger(&pair[1])?,
            "特徴" => card.traits = parse_traits(&inner_text),
            "テキスト" => card.text = parse_text(&inner_text),
            "フレーバー" => card.flavor_text = parse_text(&inner_text),
            "イラストレーター" => card.illustrator = parse_text(&inner_text),
            _ => unreachable!("unrecognized key: {:?}", pair[0].get_inner_text()),
        }
    }
    Ok(())
}

fn parse_text(text: &str) -> String {
    text.trim().replace("\u{3000}", " ")
}

fn parse_number(text: &str) -> Result<u16, Box<dyn Error>> {
    if text.chars().all(|c| c.is_digit(10)) {
        Ok(text.parse()?)
    } else {
        Ok(0)
    }
}

fn parse_side(elem: &Element) -> Result<WSCardSide, Box<dyn Error>> {
    let image = elem.find_element("img")?;
    let side = match image.get_attribute_value("src")? {
        Some(str) => match str.split("/").last().unwrap() {
            "w.gif" => WSCardSide::Weiß,
            "s.gif" => WSCardSide::Schwarz,
            _ => unreachable!("Unknown side"),
        },
        None => return Err("No src attribute found for image".into()),
    };
    Ok(side)
}

fn parse_card_type(text: &str) -> Result<WSCardType, Box<dyn Error>> {
    let card_type = match parse_text(text).as_str() {
        "キャラ" => WSCardType::Character,
        "イベント" => WSCardType::Event,
        "クライマックス" => WSCardType::Climax,
        _ => unreachable!("Unknown card type"),
    };
    Ok(card_type)
}

fn parse_color(elem: &Element) -> Result<WSCardColor, Box<dyn Error>> {
    let image = elem.find_element("img")?;
    let side = match image.get_attribute_value("src")? {
        Some(str) => match str.split("/").last().unwrap() {
            "red.gif" => WSCardColor::Red,
            "yellow.gif" => WSCardColor::Yellow,
            "blue.gif" => WSCardColor::Blue,
            "green.gif" => WSCardColor::Green,
            "purple.gif" => WSCardColor::Purple,
            _ => unreachable!("Unknown color"),
        },
        None => return Err("No src attribute found for image".into()),
    };
    Ok(side)
}

fn parse_soul(elem: &Element) -> Result<usize, Box<dyn Error>> {
    let souls = elem.find_elements("image")?;
    Ok(souls.len())
}

fn parse_trigger(elem: &Element) -> Result<WSCardTrigger, Box<dyn Error>> {
    let triggers = elem.find_elements("img")?;
    if triggers.is_empty() {
        return Ok(WSCardTrigger::None);
    };
    let trigger = match triggers
        .last()
        .unwrap()
        .get_attribute_value("src")?
        .unwrap()
        .split("/")
        .last()
        .unwrap()
    {
        "soul.gif" => {
            if triggers.len() == 2 {
                WSCardTrigger::DoubleSoul
            } else {
                WSCardTrigger::Soul
            }
        }
        "stock.gif" => WSCardTrigger::Pool,
        "salvage.gif" => WSCardTrigger::Comeback,
        "bounce.gif" => WSCardTrigger::Return,
        "draw.gif" => WSCardTrigger::Draw,
        "treasure.gif" => WSCardTrigger::Treasure,
        "shot.gif" => WSCardTrigger::Shot,
        "gate.gif" => WSCardTrigger::Gate,
        "standby.gif" => WSCardTrigger::Standby,
        "choice.gif" => WSCardTrigger::Choice,
        _ => WSCardTrigger::None,
    };
    Ok(trigger)
}

fn parse_traits(text: &str) -> Vec<String> {
    text.split("・")
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>()
}
