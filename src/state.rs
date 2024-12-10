use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    LoadingSave,
    GameSolarSystem,
    GameUniverseMap,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainMenuSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadingSaveSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniverseMapSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SolarSystemSet;
