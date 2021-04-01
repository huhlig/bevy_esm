use bevy_core::{Timer, Time};
use bevy_ecs::{Command, Res};
use bevy_reflect::TypeUuid;
use bevy_utils::Duration;

/// Stategraph
/// A Stateless representation of an Entities possible states.
#[derive(TypeUuid)]
#[uuid = "06ec0d9f-2762-45b9-a393-1aad38334993"]
pub struct StateGraph {
    states: Vec<State>,
}

impl StateGraph {
    /// Create a new State, This should be wrapped in friendly APIs
    pub fn add_state(&mut self, name: &str, commands: Vec<Box<dyn Command>>) -> StateId {
        let state_id = StateId(self.states.len());
        self.states.push(State {
            id: state_id,
            name: name.to_string(),
            commands,
            exits: vec![],
        });
        state_id
    }
    /// Get current State,
    pub fn get_state(&self, id: StateId) -> Option<&State> {
        self.states.get(id.0)
    }
    pub fn get_state_mut(&mut self, id: StateId) -> Option<&mut State> {
        self.states.get_mut(id.0)
    }
    pub fn to_dot<T: std::io::Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(writer, "digraph state_graph {{");
        for state in &self.states {
            writeln!(writer, "    {} [label=\"{}\"]", state.id.0, state.name);
            for transition in &state.exits {
                writeln!(writer, "    {}->{} [label=\"{}\"]", state.id.0, transition.target.0, state.name);
            }
        }
        writeln!(writer, "}}")
    }
}

impl Default for StateGraph {
    fn default() -> Self {
        StateGraph { states: Vec::new() }
    }
}

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct StateId(usize);

/// State
pub struct State {
    /// State ID
    pub(crate) id: StateId,
    /// State Name
    pub(crate) name: String,
    /// Commands to run upon entering state
    pub(crate) commands: Vec<Box<dyn Command>>,
    /// Exits to other States
    pub(crate) exits: Vec<StateTransition>,
}


/// State Transition
pub struct StateTransition {
    /// Destination State ID
    pub(crate) target: StateId,
    /// Commands to run on transition
    pub(crate) commands: Vec<Box<dyn Command>>,
    /// Trigger condition
    pub(crate) condition: Box<dyn Condition>,
}

pub trait Condition: Send + Sync {
    fn evaluate(&self) -> bool;
}