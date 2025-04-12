use async_trait::async_trait;
use pumpkin_data::item::Item;
use pumpkin_util::GameMode;

use crate::{
    entity::player::Player,
    item::pumpkin_item::{ItemMetadata, PumpkinItem},
};

pub struct FoodItem;

impl ItemMetadata for FoodItem {
    fn ids() -> Box<[u16]> {
        // Add the IDs of food items here
        vec![
            Item::APPLE.id,
            Item::BREAD.id,
            Item::CARROT.id,
            Item::COOKED_BEEF.id,
            Item::COOKED_CHICKEN.id,
            Item::COOKED_MUTTON.id,
            Item::COOKED_PORKCHOP.id,
            Item::COOKED_RABBIT.id,
            Item::COOKIE.id,
            Item::GOLDEN_APPLE.id,
            Item::MELON_SLICE.id,
            Item::MUSHROOM_STEW.id,
            Item::POTATO.id,
            Item::PUMPKIN_PIE.id,
            Item::RABBIT_STEW.id,
            Item::BEEF.id,
            Item::CHICKEN.id,
            Item::MUTTON.id,
            Item::PORKCHOP.id,
            Item::RABBIT.id,
            Item::ROTTEN_FLESH.id,
            Item::SPIDER_EYE.id,
            Item::SUSPICIOUS_STEW.id,
        ]
        .into_boxed_slice()
    }
}

#[async_trait]
impl PumpkinItem for FoodItem {
    async fn normal_use(&self, item: &Item, player: &Player) {
        if player.hunger_manager.level.load() < 20 {
            player.hunger_manager.consume_food(item).await;
            // Trigger eating animations and sounds here
        }
    }
}
