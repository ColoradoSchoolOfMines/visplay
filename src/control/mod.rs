mod hooks;
pub use self::hooks::ControlHooks;

mod shell;

use serde::{Serialize, Deserialize};

pub enum Intent {
    Success {
        next_node: usize,
        wait: Option<f32>,
    },
    Again {
        wait_hint: f32,
    },
    Exit,
}

pub struct EventMeta {
    past_wait: f32,
    again_counter: u32,
}

pub trait Episode: Deserialize + Serialize {
    type Node: Deserialize + Serialize + Node;
    type State: EpisodeState<Spec=Self>;

    fn get_node(&self, index: usize) -> &Self::Node;

    fn event(&self, node: &Self::Node, state: &mut Self::State, meta: EventMeta) -> Intent;

    fn on_exit(&self, state: Self::State);
}

pub trait EpisodeState {
    type Spec: Episode + ?Sized;
}

pub trait Node {
    fn takes_early(&self) -> bool;
}