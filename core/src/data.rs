use crate::creature::library::CreatureLibrary;

pub mod config;

#[derive(Debug, Default)]
pub struct GameData {
    pub config: config::Config,
    pub creatures: CreatureLibrary,
}
