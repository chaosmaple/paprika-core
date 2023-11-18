use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) enum WSCardType {
    Character,
    Event,
    Climax,
}

impl Default for WSCardType {
    fn default() -> Self {
        WSCardType::Character
    }
}

#[derive(Debug, Serialize)]
pub(crate) enum WSCardSide {
    Weiß,
    Schwarz,
}

impl Default for WSCardSide {
    fn default() -> Self {
        WSCardSide::Weiß
    }
}

#[derive(Debug, Serialize)]
pub(crate) enum WSCardColor {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Colorless,
}

impl Default for WSCardColor {
    fn default() -> Self {
        WSCardColor::Colorless
    }
}

#[derive(Debug, Serialize)]
pub(crate) enum WSCardTrigger {
    None,
    Soul,
    DoubleSoul,
    Pool,
    Comeback,
    Return,
    Draw,
    Treasure,
    Shot,
    Gate,
    Choice,
    Standby,
}

impl Default for WSCardTrigger {
    fn default() -> Self {
        WSCardTrigger::None
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct WSCard {
    pub image: String,
    pub card_name: String,
    pub card_name_kana: String,
    pub card_no: String,
    pub product: String,
    pub expansion: String,
    pub expansion_id: String,
    pub rarity: String,
    pub side: WSCardSide,
    pub card_type: WSCardType,
    pub color: WSCardColor,
    pub level: u16,
    pub cost: u16,
    pub power: u16,
    pub soul: u16,
    pub trigger: WSCardTrigger,
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
            side: WSCardSide::Weiß,
            card_type: WSCardType::Character,
            color: WSCardColor::Red,
            level: 0,
            cost: 0,
            power: 0,
            soul: 0,
            trigger: WSCardTrigger::None,
            traits: vec![],
            text: String::new(),
            flavor_text: String::from("-"),
            illustrator: String::from("-"),
        }
    }
}
