use std::error::Error;

use super::types::WSCard;
use ws_db::db::{ws_expansion, ws_product, PrismaClient};

pub(crate) async fn insert_ws_card(client: &PrismaClient, card_data: &WSCard) -> Result<(), Box<dyn Error>> {
    client
        ._transaction()
        .run(|client| async move {
            let product = client
                .ws_product()
                .upsert(
                    ws_product::product::equals(card_data.product.clone()),
                    ws_product::create(card_data.product.clone(), vec![]),
                    vec![],
                )
                .exec()
                .await?;

            let expansion = client
                .ws_expansion()
                .upsert(
                    ws_expansion::expansion_id::equals(card_data.expansion_id.clone()),
                    ws_expansion::create(
                        card_data.expansion_id.clone(),
                        card_data.expansion.clone(),
                        vec![],
                    ),
                    vec![],
                )
                .exec()
                .await?;

            client
                .ws_card()
                .create(
                    card_data.image.clone(),
                    card_data.card_name.clone(),
                    card_data.card_no.clone(),
                    ws_product::product::equals(card_data.product.clone()),
                    ws_expansion::expansion_id::equals(card_data.expansion_id.clone()),
                    card_data.rarity.clone(),
                    card_data.side,
                    card_data.card_type,
                    card_data.color,
                    card_data.level,
                    card_data.cost,
                    card_data.power,
                    card_data.soul,
                    card_data.trigger,
                    vec![],
                )
                .exec()
                .await
        })
        .await?;

    Ok(())
}
