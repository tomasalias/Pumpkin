use constants::{DAMAGE, MAX_DAMAGE};
use pumpkin_data::item::Item;
use pumpkin_data::tag::{RegistryKey, get_tag_values};
use pumpkin_nbt::compound::NbtCompound;
use std::hash::Hash;

mod categories;
mod constants;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
/// Item Rarity
pub enum Rarity {
    Common,
    UnCommon,
    Rare,
    Epic,
}

#[derive(Clone, Debug, Copy)]
pub struct ItemStack {
    pub item_count: u8,
    pub item: Item,
}

impl Hash for ItemStack {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.item_count.hash(state);
        self.item.id.hash(state);
    }
}

/*
impl PartialEq for ItemStack {
    fn eq(&self, other: &Self) -> bool {
        self.item.id == other.item.id
    }
} */

impl ItemStack {
    pub const EMPTY: ItemStack = ItemStack {
        item_count: 0,
        item: Item::AIR,
    };

    pub fn new(item_count: u8, item: &'static Item) -> Self {
        Self { item_count, item: item.clone() }
    }

    pub fn damage_item(&mut self) {
        let components = &mut self.item.components;

        if let (Some(_damage), Some(_max_damage)) = (components.damage, components.max_damage) {
            if _max_damage == 0 {
                return;
            }

            // TODO: we probably have to consider the unbreakable enchantment here
            if _damage >= _max_damage {
                return;
            }

            components.damage = Some(_damage + 1);
        }
    }

    pub fn is_broken(&self) -> bool {
        self.is_damageable() && self.get_damage() >= self.get_max_damage()
    }

    pub fn is_damageable(&self) -> bool {
        let components = self.item.components;

        components.contains(DAMAGE) && components.contains(MAX_DAMAGE)
    }

    pub fn is_damaged(&self) -> bool {
        self.is_damageable() && self.item.components.damage.unwrap() > 0
    }

    pub fn get_damage(&self) -> u16 {
        self.item.components.damage.unwrap_or_default()
    }

    pub fn get_max_damage(&self) -> u16 {
        self.item.components.max_damage.unwrap_or_default()
    }

    pub fn get_max_stack_size(&self) -> u8 {
        self.item.components.max_stack_size
    }

    pub fn get_item(&self) -> &Item {
        if self.is_empty() {
            &Item::AIR
        } else {
            &self.item
        }
    }

    pub fn is_stackable(&self) -> bool {
        self.get_max_stack_size() > 1 // TODO: && (!this.isDamageable() || !this.isDamaged());
    }

    pub fn is_empty(&self) -> bool {
        self.item_count == 0 || self.item.id == Item::AIR.id
    }

    pub fn split(&mut self, amount: u8) -> Self {
        let min = amount.min(self.item_count);
        let stack = self.copy_with_count(min);
        self.decrement(min);
        stack
    }

    pub fn copy_with_count(&self, count: u8) -> Self {
        let mut stack = *self;
        stack.item_count = count;
        stack
    }

    pub fn set_count(&mut self, count: u8) {
        self.item_count = count;
    }

    pub fn decrement(&mut self, amount: u8) {
        self.item_count = self.item_count.saturating_sub(amount);
    }

    pub fn increment(&mut self, amount: u8) {
        self.item_count = self.item_count.saturating_add(amount);
    }

    pub fn are_items_and_components_equal(&self, other: &Self) -> bool {
        self.item == other.item //TODO: && self.item.components == other.item.components
    }

    pub fn are_equal(&self, other: &Self) -> bool {
        self.item_count == other.item_count && self.are_items_and_components_equal(other)
    }

    /// Determines the mining speed for a block based on tool rules.
    /// Direct matches return immediately, tagged blocks are checked separately.
    /// If no match is found, returns the tool's default mining speed or `1.0`.
    pub fn get_speed(&self, block: &str) -> f32 {
        // No tool? Use default speed
        let Some(tool) = &self.item.components.tool else {
            return 1.0;
        };

        for rule in tool.rules {
            // Skip if speed is not set
            let Some(speed) = rule.speed else {
                continue;
            };

            for entry in rule.blocks {
                if entry.eq(&block) {
                    return speed;
                }

                if entry.starts_with('#') {
                    // Check if block is in the tag group
                    if let Some(blocks) =
                        get_tag_values(RegistryKey::Block, entry.strip_prefix('#').unwrap())
                    {
                        if blocks.contains(&block) {
                            return speed;
                        }
                    }
                }
            }
        }
        // Return default mining speed if no match is found
        tool.default_mining_speed.unwrap_or(1.0)
    }

    /// Determines if a tool is valid for block drops based on tool rules.
    /// Direct matches return immediately, while tagged blocks are checked separately.
    pub fn is_correct_for_drops(&self, block: &str) -> bool {
        // Return false if no tool component exists
        let Some(tool) = &self.item.components.tool else {
            return false;
        };

        for rule in tool.rules {
            // Skip rules without a drop condition
            let Some(correct_for_drops) = rule.correct_for_drops else {
                continue;
            };

            for entry in rule.blocks {
                if entry.eq(&block) {
                    return correct_for_drops;
                }

                if entry.starts_with('#') {
                    // Check if block exists within the tag group
                    if let Some(blocks) =
                        get_tag_values(RegistryKey::Block, entry.strip_prefix('#').unwrap())
                    {
                        if blocks.contains(&block) {
                            return correct_for_drops;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn write_item_stack(&self, compound: &mut NbtCompound) {
        // Minecraft 1.21.4 uses "id" as string with namespaced ID (minecraft:diamond_sword)
        compound.put_string("id", format!("minecraft:{}", self.item.registry_key));
        compound.put_int("count", self.item_count as i32);

        // Create a tag compound for additional data
        let mut tag = NbtCompound::new();

        // TODO: Store custom data like enchantments, display name, etc.
        if let Some(damage) = self.item.components.damage {
            tag.put_int("damage", damage as i32);
        }

        if let Some(max_damage) = self.item.components.max_damage {
            tag.put_int("max_damage", max_damage as i32);
        }

        compound.put_component("components", tag);
    }

    pub fn read_item_stack(compound: &NbtCompound) -> Option<Self> {
        // Get ID, which is a string like "minecraft:diamond_sword"
        let full_id = compound.get_string("id")?;

        // Remove the "minecraft:" prefix if present
        let registry_key = full_id.strip_prefix("minecraft:").unwrap_or(full_id);

        let item = Item::from_registry_key(registry_key)?;

        let count = compound.get_int("count")? as u8;

        let mut item_stack = Self::new(count, item);

        let item = &mut item_stack.item;

        // TODO: Process additional components like damage, enchantments, etc.
        if let Some(_tag) = compound.get_compound("components") {
            if let Some(_damage) = _tag.get_int("damage") {
                item.components.damage = Some(_damage as u16);
            }

            if let Some(_max_damage) = _tag.get_int("max_damage") {
                item.components.max_damage = Some(_max_damage as u16);
            }
        }

        Some(item_stack)
    }
}
