use std::time::Duration;

use crate::entity::player::Player;
use crate::item::pumpkin_item::{ItemMetadata, PumpkinItem};
use async_trait::async_trait;
use pumpkin_data::item::Item;
use pumpkin_data::sound::Sound;
use pumpkin_protocol::client::play::{MetaDataType, Metadata};
use pumpkin_world::entity::entity_data_flags::DATA_LIVING_ENTITY_FLAGS;

pub struct FoodItem;

impl ItemMetadata for FoodItem {
    fn ids() -> Box<[u16]> {
        // All food items - we'll handle them all with this one handler
        vec![
            Item::APPLE.id,
            Item::BREAD.id,
            Item::CARROT.id,
            Item::POTATO.id,
            Item::BAKED_POTATO.id,
            Item::BEETROOT.id,
            Item::DRIED_KELP.id,
            Item::BEEF.id,
            Item::COOKED_BEEF.id,
            Item::PORKCHOP.id,
            Item::COOKED_PORKCHOP.id,
            Item::MUTTON.id,
            Item::COOKED_MUTTON.id,
            Item::CHICKEN.id,
            Item::COOKED_CHICKEN.id,
            Item::COD.id,
            Item::COOKED_COD.id,
            Item::SALMON.id,
            Item::COOKED_SALMON.id,
            Item::TROPICAL_FISH.id,
            Item::PUFFERFISH.id,
            Item::CHORUS_FRUIT.id,
            Item::SWEET_BERRIES.id,
            Item::GLOW_BERRIES.id,
            Item::MELON_SLICE.id,
            Item::GOLDEN_APPLE.id,
            Item::ENCHANTED_GOLDEN_APPLE.id,
            Item::GOLDEN_CARROT.id,
            Item::SPIDER_EYE.id,
            Item::POISONOUS_POTATO.id,
            Item::ROTTEN_FLESH.id,
            Item::COOKIE.id,
            Item::PUMPKIN_PIE.id,
            Item::CAKE.id,
        ]
        .into_boxed_slice()
    }
}

#[async_trait]
impl PumpkinItem for FoodItem {
    async fn normal_use(&self, item: &Item, player: &Player) {
        if let Some(food) = &item.components.food {
            // Check if player can consume the food
            if food.can_always_eat.unwrap_or(false) || player.hunger_manager.level.load() < 20 {
                self.begin_eating(item, player).await;
            }
        }
    }
}

impl FoodItem {
    pub fn _get_eat_time(_item: &Item) -> Duration {
        // Default eating time is 1.6 seconds (32 ticks)
        Duration::from_millis(1600)
    }
    /// Begin the eating animation and sound for this food item
    pub async fn begin_eating(&self, item: &Item, player: &Player) {
        // Play eating sound
        let sound = Sound::EntityGenericEat; // Default eating sound

        player
            .world()
            .await
            .play_sound(
                sound,
                pumpkin_data::sound::SoundCategory::Players,
                &player.living_entity.entity.pos.load(),
            )
            .await;

        // For now, immediately consume the food instead of waiting
        // TODO: Implement proper eating timer with animation
        Self::finish_eating(item, player).await;
    }

    /// Set the player in eating state with the specified item and duration
    /// TODO: Implement proper eating animation with timer
    pub async fn _set_eating(&self, item: &Item, player: &Player, _duration: Duration) {
        // Set entity metadata to show eating animation
        // The eating animation uses the DATA_LIVING_ENTITY_FLAGS field
        // Bit 0x01 is for using item (eating/drinking/blocking)
        let current_flags = 0u8; // TODO: Get current flags properly
        let eating_flags = current_flags | 0x01; // Set eating bit

        player
            .living_entity
            .entity
            .send_meta_data(&[Metadata::new(
                DATA_LIVING_ENTITY_FLAGS,
                MetaDataType::Byte,
                eating_flags,
            )])
            .await;

        // TODO: Set up proper timer to finish eating after duration
        // For now we'll just immediately finish eating
        Self::finish_eating(item, player).await;
    }

    /// Complete the eating process - consume the food and apply effects
    pub async fn finish_eating(item: &Item, player: &Player) {
        if let Some(food) = &item.components.food {
            // Feed the player
            player.hunger_manager.feed(food.nutrition, food.saturation);

            // Send updated health/hunger to client
            player.send_health().await;

            // Remove eating animation
            let current_flags = 0u8; // TODO: Get current flags properly  
            let not_eating_flags = current_flags & !0x01; // Clear eating bit

            player
                .living_entity
                .entity
                .send_meta_data(&[Metadata::new(
                    DATA_LIVING_ENTITY_FLAGS,
                    MetaDataType::Byte,
                    not_eating_flags,
                )])
                .await;

            // Consume the item (reduce count by 1)
            let held_item = player.inventory.held_item();
            let mut item_stack = held_item.lock().await;
            if !item_stack.is_empty() && item_stack.item.id == item.id {
                item_stack.decrement(1);
            }

            // TODO: Apply food effects if any
            // TODO: Play burp sound if hunger was full
        }
    }
}
