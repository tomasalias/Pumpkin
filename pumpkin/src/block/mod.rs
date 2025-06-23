use blocks::bamboo::BambooBlock;
use blocks::barrel::BarrelBlock;
use blocks::bed::BedBlock;
use blocks::cactus::CactusBlock;
use blocks::carpet::{CarpetBlock, MossCarpetBlock, PaleMossCarpetBlock};
use blocks::command::CommandBlock;
use blocks::dirt_path::DirtPathBlock;
use blocks::doors::DoorBlock;
use blocks::end_portal::EndPortalBlock;
use blocks::end_portal_frame::EndPortalFrameBlock;
use blocks::farmland::FarmLandBlock;
use blocks::fence_gates::FenceGateBlock;
use blocks::fences::FenceBlock;
use blocks::fire::fire::FireBlock;
use blocks::fire::soul_fire::SoulFireBlock;
use blocks::glass_panes::GlassPaneBlock;
use blocks::grindstone::GrindstoneBlock;
use blocks::iron_bars::IronBarsBlock;
use blocks::logs::LogBlock;
use blocks::nether_portal::NetherPortalBlock;
use blocks::note::NoteBlock;
use blocks::piston::piston::PistonBlock;
use blocks::piston::piston_extension::PistonExtensionBlock;
use blocks::piston::piston_head::PistonHeadBlock;
use blocks::plant::bush::BushBlock;
use blocks::plant::dry_vegetation::DryVegetationBlock;
use blocks::plant::flower::FlowerBlock;
use blocks::plant::flowerbed::FlowerbedBlock;
use blocks::plant::leaf_litter::LeafLitterBlock;
use blocks::plant::lily_pad::LilyPadBlock;
use blocks::plant::mushroom_plant::MushroomPlantBlock;
use blocks::plant::sapling::SaplingBlock;
use blocks::plant::short_plant::ShortPlantBlock;
use blocks::plant::tall_plant::TallPlantBlock;
use blocks::pumpkin::PumpkinBlock;
use blocks::redstone::buttons::ButtonBlock;
use blocks::redstone::comparator::ComparatorBlock;
use blocks::redstone::observer::ObserverBlock;
use blocks::redstone::pressure_plate::plate::PressurePlateBlock;
use blocks::redstone::pressure_plate::weighted::WeightedPressurePlateBlock;
use blocks::redstone::rails::activator_rail::ActivatorRailBlock;
use blocks::redstone::rails::detector_rail::DetectorRailBlock;
use blocks::redstone::rails::powered_rail::PoweredRailBlock;
use blocks::redstone::rails::rail::RailBlock;
use blocks::redstone::redstone_block::RedstoneBlock;
use blocks::redstone::redstone_lamp::RedstoneLamp;
use blocks::redstone::redstone_torch::RedstoneTorchBlock;
use blocks::redstone::redstone_wire::RedstoneWireBlock;
use blocks::redstone::repeater::RepeaterBlock;
use blocks::redstone::target_block::TargetBlock;
use blocks::redstone::tripwire::TripwireBlock;
use blocks::redstone::tripwire_hook::TripwireHookBlock;
use blocks::sea_pickles::SeaPickleBlock;
use blocks::signs::SignBlock;
use blocks::slabs::SlabBlock;
use blocks::sponge::{SpongeBlock, WetSpongeBlock};
use blocks::stairs::StairBlock;
use blocks::sugar_cane::SugarCaneBlock;
use blocks::torches::TorchBlock;
use blocks::trapdoor::TrapDoorBlock;
use blocks::vine::VineBlock;
use blocks::walls::WallBlock;
use blocks::{
    chest::ChestBlock, furnace::FurnaceBlock, redstone::lever::LeverBlock, tnt::TNTBlock,
};
use fluid::lava::FlowingLava;
use fluid::water::FlowingWater;
use loot::LootTableExt;
use pumpkin_data::block_properties::Integer0To15;
use pumpkin_data::{Block, BlockState};

use pumpkin_util::math::position::BlockPos;
use pumpkin_util::random::{RandomGenerator, get_seed, xoroshiro128::Xoroshiro};
use pumpkin_world::BlockStateId;

use crate::block::blocks::plant::roots::RootsBlock;
use crate::block::loot::LootContextParameters;
use crate::block::registry::BlockRegistry;
use crate::world::World;
use crate::{block::blocks::crafting_table::CraftingTableBlock, entity::player::Player};
use crate::{block::blocks::jukebox::JukeboxBlock, entity::experience_orb::ExperienceOrbEntity};
use std::sync::Arc;

pub mod blocks;
mod fluid;
pub mod loot;
pub mod pumpkin_block;
pub mod pumpkin_fluid;
pub mod registry;

#[must_use]
pub fn default_registry() -> Arc<BlockRegistry> {
    let mut manager = BlockRegistry::default();

    // Blocks
    manager.register(BedBlock);
    manager.register(SaplingBlock);
    manager.register(CactusBlock);
    manager.register(CarpetBlock);
    manager.register(MossCarpetBlock);
    manager.register(PaleMossCarpetBlock);
    manager.register(ChestBlock);
    manager.register(CraftingTableBlock);
    manager.register(DirtPathBlock);
    manager.register(DoorBlock);
    manager.register(FarmLandBlock);
    manager.register(FenceGateBlock);
    manager.register(FenceBlock);
    manager.register(FurnaceBlock);
    manager.register(GlassPaneBlock);
    manager.register(GrindstoneBlock);
    manager.register(IronBarsBlock);
    manager.register(JukeboxBlock);
    manager.register(LogBlock);
    manager.register(BambooBlock);
    manager.register(SignBlock);
    manager.register(SlabBlock);
    manager.register(SpongeBlock);
    manager.register(WetSpongeBlock);
    manager.register(StairBlock);
    manager.register(ShortPlantBlock);
    manager.register(DryVegetationBlock);
    manager.register(LilyPadBlock);
    manager.register(SugarCaneBlock);
    manager.register(VineBlock);
    manager.register(TNTBlock);
    manager.register(BushBlock);
    manager.register(FlowerBlock);
    manager.register(TorchBlock);
    manager.register(TrapDoorBlock);
    manager.register(MushroomPlantBlock);
    manager.register(FlowerbedBlock);
    manager.register(LeafLitterBlock);
    manager.register(WallBlock);
    manager.register(RootsBlock);
    manager.register(NetherPortalBlock);
    manager.register(TallPlantBlock);
    manager.register(NoteBlock);
    manager.register(PumpkinBlock);
    manager.register(CommandBlock);
    manager.register(PressurePlateBlock);
    manager.register(WeightedPressurePlateBlock);
    manager.register(EndPortalBlock);
    manager.register(EndPortalFrameBlock);
    manager.register(SeaPickleBlock);

    // Fire
    manager.register(SoulFireBlock);
    manager.register(FireBlock);

    // Redstone
    manager.register(ButtonBlock);
    manager.register(LeverBlock);
    manager.register(ObserverBlock);
    manager.register(TripwireBlock);
    manager.register(TripwireHookBlock);

    // Piston
    manager.register(PistonBlock);
    manager.register(PistonExtensionBlock);
    manager.register(PistonHeadBlock);

    manager.register(RedstoneBlock);
    manager.register(RedstoneLamp);
    manager.register(RedstoneTorchBlock);
    manager.register(RedstoneWireBlock);
    manager.register(RepeaterBlock);
    manager.register(ComparatorBlock);
    manager.register(TargetBlock);
    manager.register(BarrelBlock);

    // Rails
    manager.register(RailBlock);
    manager.register(ActivatorRailBlock);
    manager.register(DetectorRailBlock);
    manager.register(PoweredRailBlock);

    // Fluids
    manager.register_fluid(FlowingWater);
    manager.register_fluid(FlowingLava);
    Arc::new(manager)
}

#[derive(Clone)]
pub struct BlockEvent {
    pub pos: BlockPos,
    pub r#type: u8,
    pub data: u8,
}

pub async fn drop_loot(
    world: &Arc<World>,
    block: &Block,
    pos: &BlockPos,
    experience: bool,
    params: LootContextParameters,
) {
    if let Some(loot_table) = &block.loot_table {
        for stack in loot_table.get_loot(params) {
            world.drop_stack(pos, stack).await;
        }
    }

    if experience {
        if let Some(experience) = &block.experience {
            let mut random = RandomGenerator::Xoroshiro(Xoroshiro::from_seed(get_seed()));
            let amount = experience.experience.get(&mut random);
            // TODO: Silk touch gives no exp
            if amount > 0 {
                ExperienceOrbEntity::spawn(world, pos.to_f64(), amount as u32).await;
            }
        }
    }
}

pub async fn calc_block_breaking(player: &Player, state: &BlockState, block_name: &str) -> f32 {
    let hardness = state.hardness;
    #[expect(clippy::float_cmp)]
    if hardness == -1.0 {
        // unbreakable
        return 0.0;
    }
    let i = if player.can_harvest(state, block_name).await {
        30
    } else {
        100
    };

    player.get_mining_speed(block_name).await / hardness / i as f32
}

#[derive(PartialEq)]
pub enum BlockIsReplacing {
    Itself(BlockStateId),
    Water(Integer0To15),
    Other,
    None,
}

impl BlockIsReplacing {
    #[must_use]
    /// Returns true if the block was a water source block.
    pub fn water_source(&self) -> bool {
        match self {
            // Level 0 means the water is a source block
            Self::Water(level) => *level == Integer0To15::L0,
            _ => false,
        }
    }
}
