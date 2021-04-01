use bevy_asset::{Handle, Assets};
use crate::graph::{
    StateGraph,
    StateId,
};
use bevy_ecs::{Res, Query, Commands};

pub struct EntityState {
    state_graph_handle: Handle<StateGraph>,
    current_state: StateId,
}

fn esm_update_system(
    state_graphs: Res<Assets<StateGraph>>,
    mut commands: &mut Commands,
    mut query: Query<(&mut EntityState)>,
) {
    for (mut entity_state) in query.iter_mut() {
        if let Some(state_graph) = state_graphs.get(&entity_state.state_graph_handle) {
            if let Some(current_state) = state_graph.get_state(entity_state.current_state) {
                for exit in &current_state.exits {
                    if exit.condition.evaluate() {
                        entity_state.current_state = exit.target;
                        for command in &exit.commands {
                            commands.add_command_boxed(*command);
                        }
                    }
                }
            }
        }
    }
}
