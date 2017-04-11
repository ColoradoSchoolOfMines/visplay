use super::{Intent, EventMeta};
use super::Intent::*;

use std::process::Child;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct ShellSpec {
    program: String,
    kill_on_end: bool,
    args: Vec<String>,
    nodes: Vec<ShellNode>,
}

impl super::Episode for ShellSpec {
    type Node = ShellNode;
    type State = ShellState;

    fn get_node(&self, index: usize) -> &ShellNode { &self.nodes[index] }

    fn event(&self, node: &ShellNode, state: &mut ShellState, meta: EventMeta) -> Intent {
        if meta.again_counter == 0 {
            node.start(self, state);
        }
        node.event(self, state, meta)
    }

    fn on_exit(&self, mut state: ShellState) {
        if self.kill_on_end {
            if let Err(e) = state.program.kill() {
                error!("Could not kill child process, letting it live: {}", e)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
enum ShellNode {
    PipeToStdin {
        text: String,
        simulate_typing: Option<f32>,
        on_end: usize,
        on_error: Option<usize>,
    },
    MatchStdout {
        regex: String,
        on_match: usize,
        on_eof: Option<usize>,
    },
    HideWindow,
    ShowWindow,
    ExitEarly,
}

impl ShellNode {
    fn start(&self, _: &ShellSpec, state: &mut ShellState) {
        // we need to print from the beginning
        state.print_index = 0;
    }

    fn event(&self, _: &ShellSpec, state: &mut ShellState, _: EventMeta) -> Intent {
        use self::ShellNode::*;

        match *self {
            ExitEarly => Exit,
            PipeToStdin { ref text, simulate_typing, on_end, on_error } => {
                // create error intent
                let error_intent = on_error
                        .map(|n| Success { next_node: n, wait: None })
                        .unwrap_or(Exit);

                // how far have we printed the text already?
                let mut index = state.print_index;

                // get stdin
                let stdin = state.program.stdin.as_mut();
                if stdin.is_none() { 
                    error!("Child process does not have stdin");
                    return error_intent;
                };
                let stdin = stdin.unwrap();

                // get text slice
                let slice = match simulate_typing {
                    Some(_) => &text[index..index + 1],
                    None => &text[index..],
                };

                // write
                if let Err(e) = stdin.write_all(text[index..index + 1].as_bytes()) {
                    error!("Could not write to child process stdin: {}", e);
                    return error_intent;
                }
                index += slice.len();

                // check at end
                let out = if index == text.len() { Success { next_node: on_end, wait: None } } // yes, next node
                else { Again { wait_hint: simulate_typing.unwrap_or(0.) } }; // not at end, continue printing

                // set print_index to remember the additional chars sent
                state.print_index = index;

                // return intent
                out
            },
            _ => Exit,
        }
    }
}

impl super::Node for ShellNode {
    fn takes_early(&self) -> bool {
        false
    }
}

struct ShellState {
    print_index: usize,
    program: Child,
}

impl super::EpisodeState for ShellState {
    type Spec = ShellSpec;
}