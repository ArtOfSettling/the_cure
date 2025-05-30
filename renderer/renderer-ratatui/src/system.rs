use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Res, ResMut};
use input_api::{PendingPlayerInputAction, PlayerInputAction};
use renderer_api::{ClientGameState, ClientHistoryState, RendererResource};
use shared::{ConnectionStateResource, PendingPlayerAction};

pub fn render_system(
    mut render_resource: ResMut<RendererResource>,
    game_state: Res<ClientGameState>,
    history_state: Res<ClientHistoryState>,
    pending_player_input_action: ResMut<PendingPlayerInputAction>,
    pending_player_action: ResMut<PendingPlayerAction>,
    connection_state_resource: Res<ConnectionStateResource>,
    mut exit_writer: EventWriter<AppExit>,
) {
    if let Some(PlayerInputAction::Quit) = pending_player_input_action.0.clone() {
        exit_writer.send(AppExit::Success);
    }

    render_resource.renderer.render(
        &game_state,
        &history_state,
        pending_player_input_action,
        pending_player_action,
        connection_state_resource,
    );
}
