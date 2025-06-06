use bevy::prelude::Resource;

#[derive(Resource)]
pub struct InputResource {}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerInputAction(pub Option<PlayerInputAction>);

#[derive(Clone, Debug)]
pub enum PlayerInputAction {
    DoNothing,
    Quit,

    CreateNewGame,
    ListGames,
    DeleteGame,

    LaunchPRCampaign,
    SelectEmployeeToFire,
    SelectEmployeeToHire,
    SelectEmployeeForRaise,
    SelectEmployeeForPromotionToVP,

    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
    MenuSelect,
    MenuCommit,
    MenuBack,
    MenuChangeTab,
    MenuIncrement,
    MenuDecrement,
}
