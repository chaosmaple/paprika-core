use serde::Serialize;
use ws_db::db::{WsCardColor, WsCardSide, WsCardTrigger, WsCardType};

#[derive(Debug, Serialize)]
pub struct WSCard {
    pub image: String,
    pub card_name: String,
    pub card_name_kana: String,
    pub card_no: String,
    pub product: String,
    pub expansion: String,
    pub expansion_id: String,
    pub rarity: String,
    pub side: WsCardSide,
    pub card_type: WsCardType,
    pub color: WsCardColor,
    pub level: i32,
    pub cost: i32,
    pub power: i32,
    pub soul: i32,
    pub trigger: WsCardTrigger,
    pub traits: Vec<String>,
    pub text: String,
    pub flavor_text: String,
    pub illustrator: String,
}

impl Default for WSCard {
    fn default() -> Self {
        WSCard {
            image: String::new(),
            card_name: String::new(),
            card_name_kana: String::new(),
            card_no: String::new(),
            product: String::new(),
            expansion: String::new(),
            expansion_id: String::new(),
            rarity: String::new(),
            side: WsCardSide::W,
            card_type: WsCardType::Character,
            color: WsCardColor::Red,
            level: 0,
            cost: 0,
            power: 0,
            soul: 0,
            trigger: WsCardTrigger::None,
            traits: vec![],
            text: String::new(),
            flavor_text: String::from("-"),
            illustrator: String::from("-"),
        }
    }
}
