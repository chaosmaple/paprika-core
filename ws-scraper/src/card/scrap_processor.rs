use scraper::{ElementRef, Selector};
use std::error::Error;
use ws_db::db::{WsCardColor, WsCardSide, WsCardTrigger, WsCardType};

use super::types::*;

pub(crate) fn process_detail_section(
    detail_section: &ElementRef,
) -> Result<WSCard, Box<dyn Error>> {
    let tr_selector = Selector::parse("tr").unwrap();
    let card_details = detail_section.select(&tr_selector);
    let mut card = WSCard::default();

    for tr in card_details {
        if let Some(class) = tr.attr("class") {
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

fn process_picture_n_names(tr: &ElementRef, card: &mut WSCard) -> Result<(), Box<dyn Error>> {
    let td_slct = Selector::parse("td").unwrap();
    let img_slct = Selector::parse("img").unwrap();
    let mut tds = tr.select(&td_slct);
    card.image = tds
        .next()
        .and_then(|td| td.select(&img_slct).next())
        .and_then(|img| img.attr("src"))
        .map(str::to_owned)
        .unwrap();
    let card_names = tds.next().unwrap().inner_html();
    let mut cns = card_names.split("<br>");
    card.card_name = cns.next().unwrap().trim().to_owned();
    card.card_name_kana = cns.next().unwrap().trim().to_owned();

    Ok(())
}

fn process_row(elem: &ElementRef, card: &mut WSCard) -> Result<(), Box<dyn Error>> {
    let th_selector = Selector::parse("th").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let ths = elem.select(&th_selector);
    let tds = elem.select(&td_selector);
    for pair in ths.zip(tds) {
        let column_name = pair.0.inner_html();
        let inner_text = pair.1.inner_html();
        // println!("processing {}: {}", column_name, inner_text);
        match column_name.as_str() {
            "カード番号" => card.card_no = parse_text(&inner_text),
            "商品名" => card.product = parse_text(&inner_text),
            "ネオスタンダード区分" => card.expansion = parse_text(&inner_text),
            "作品番号" => card.expansion_id = parse_text(&inner_text),
            "レアリティ" => card.rarity = parse_text(&inner_text),
            "サイド" => card.side = parse_side(&pair.1)?,
            "種類" => card.card_type = parse_card_type(&inner_text)?,
            "色" => card.color = parse_color(&pair.1)?,
            "レベル" => card.level = parse_number(&inner_text)?,
            "コスト" => card.cost = parse_number(&inner_text)?,
            "パワー" => card.power = parse_number(&inner_text)?,
            "ソウル" => card.soul = parse_soul(&pair.1)? as i32,
            "トリガー" => card.trigger = parse_trigger(&pair.1)?,
            "特徴" => card.traits = parse_traits(&inner_text),
            "テキスト" => card.text = parse_text(&inner_text),
            "フレーバー" => card.flavor_text = parse_text(&inner_text),
            "イラストレーター" => card.illustrator = parse_text(&inner_text),
            _ => unreachable!("unrecognized key: {:?}", &column_name),
        }
    }
    Ok(())
}

fn parse_text(text: &str) -> String {
    text.trim().replace("\u{3000}", " ")
}

fn parse_number(text: &str) -> Result<i32, Box<dyn Error>> {
    if text.chars().all(|c| c.is_digit(10)) {
        Ok(text.parse()?)
    } else {
        Ok(0)
    }
}

fn parse_side(elem: &ElementRef) -> Result<WsCardSide, Box<dyn Error>> {
    let image_src = elem
        .select(&Selector::parse("img").unwrap())
        .next()
        .and_then(|img| img.attr("src"))
        .and_then(|src| src.split("/").last());
    let side = match image_src {
        Some(str) => match str {
            "w.gif" => WsCardSide::W,
            "s.gif" => WsCardSide::S,
            _ => unreachable!("Unknown side"),
        },
        None => unreachable!("None side attr"),
    };
    Ok(side)
}

fn parse_card_type(text: &str) -> Result<WsCardType, Box<dyn Error>> {
    let card_type = match parse_text(text).as_str() {
        "キャラ" => WsCardType::Character,
        "イベント" => WsCardType::Event,
        "クライマックス" => WsCardType::Climax,
        _ => unreachable!("Unknown card type"),
    };
    Ok(card_type)
}

fn parse_color(elem: &ElementRef) -> Result<WsCardColor, Box<dyn Error>> {
    let image = elem
        .select(&Selector::parse("img").unwrap())
        .next()
        .and_then(|img| img.attr("src"));
    let side = match image {
        Some(str) => match str.split("/").last().unwrap() {
            "red.gif" => WsCardColor::Red,
            "yellow.gif" => WsCardColor::Yellow,
            "blue.gif" => WsCardColor::Blue,
            "green.gif" => WsCardColor::Green,
            "purple.gif" => WsCardColor::Purple,
            _ => unreachable!("Unknown color"),
        },
        None => unreachable!("None color attr"),
    };
    Ok(side)
}

fn parse_soul(elem: &ElementRef) -> Result<usize, Box<dyn Error>> {
    let length = elem.select(&Selector::parse("img").unwrap()).count();
    Ok(length)
}

fn parse_trigger(elem: &ElementRef) -> Result<WsCardTrigger, Box<dyn Error>> {
    let selector = &Selector::parse("img").unwrap();
    let triggers = elem.select(selector);
    let tri_len = triggers.clone().count();
    if tri_len == 0 {
        return Ok(WsCardTrigger::None);
    };
    let trigger = match triggers
        .last()
        .and_then(|img| img.attr("src"))
        .and_then(|src| src.split("/").last())
    {
        Some(str) => match str {
            "soul.gif" => {
                if tri_len == 2 {
                    WsCardTrigger::DoubleSoul
                } else {
                    WsCardTrigger::Soul
                }
            }
            "stock.gif" => WsCardTrigger::Pool,
            "salvage.gif" => WsCardTrigger::Comeback,
            "bounce.gif" => WsCardTrigger::Return,
            "draw.gif" => WsCardTrigger::Draw,
            "treasure.gif" => WsCardTrigger::Treasure,
            "shot.gif" => WsCardTrigger::Shot,
            "gate.gif" => WsCardTrigger::Gate,
            "standby.gif" => WsCardTrigger::Standby,
            "choice.gif" => WsCardTrigger::Choice,
            _ => WsCardTrigger::None,
        },
        None => unreachable!("None trigger attr"),
    };
    Ok(trigger)
}

fn parse_traits(text: &str) -> Vec<String> {
    text.split("・")
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>()
}
