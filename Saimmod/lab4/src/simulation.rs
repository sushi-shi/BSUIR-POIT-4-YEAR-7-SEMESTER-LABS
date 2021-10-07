use petgraph::graph;
use petgraph::graph::{NodeIndex, Graph};

use std::collections::{HashMap};

use crate::seq::Random;

struct System {
    graph: graph::Graph<Element, ()>,
    entry: graph::NodeIndex,
    ro: f32,
    qsize: u8,
    pi1: f32,
    pi2: f32,
}

#[derive(Clone, Copy)]
enum Element {
    Generator(f32),
    Queue(usize, u8),
    Retractor(usize, f32),
}

#[derive(Clone, Copy)]
enum Message {
    IsConsumed,
    IsPending,
}

impl Element {
    fn no(&self) -> usize {
        *match self {
            Element::Generator(_) => unreachable!(), // Is always first
            Element::Queue(no, _) => no,
            Element::Retractor(no, _) => no,
        }
    }
}

struct State {
    state: [u8; 3],

    loaded: [usize; 3],
    consumed: [usize; 3],
    rejected: usize,
}


fn var36(ro: f32, qsize: u8, pi1: f32, pi2: f32) -> System {
    let mut graph = graph::Graph::new();
    let gen = graph.add_node(Element::Generator(ro));
    let que = graph.add_node(Element::Queue(0, qsize));
    let ret1 = graph.add_node(Element::Retractor(1, pi1));
    let ret2 = graph.add_node(Element::Retractor(2, pi2));

    let _ = graph.add_edge(gen, que, ());
    let _ = graph.add_edge(gen, ret2, ());
    let _ = graph.add_edge(que, ret1, ());

    System {
        graph,
        qsize,
        entry: gen,
        ro,
        pi1,
        pi2,
    }
}

// frigging list monad..
fn step(system: &System, random: &mut Random, state: State) -> State {
    if let Element::Generator(ro) = system.graph.node_weight(system.entry).unwrap() {
        let cointoss = random.random();
        let message = if cointoss < *ro {
            Message::IsConsumed
        } else {
            Message::IsPending
        };
        step_rec(&system.graph, system.entry, random, state, message).0
    } else {
        unreachable!()
    }
}

fn step_rec(
    graph: &Graph<Element, ()>,
    node: NodeIndex, 
    random: &mut Random,
    state: State, 
    message: Message,
) -> (State, Message) {
    let mut state = state;
    let cointoss = random.random(); // because I'm stupid and don't know what to do with this closure
    let mut recurse = |node, state, message| {
        let mut nodes = graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .map(|idx| (idx, graph.node_weight(idx).unwrap())) 
            .collect::<Vec<_>>();
        nodes.sort_by_key(|x| x.1.no());
        nodes.iter().map(|x| x.0).fold((state, message), |(state, message), node|
            step_rec(graph, node, random, state, message)
        )
    };

    match graph.node_weight(node).unwrap() {
        Element::Generator(_) => {
            let first_message = message;
            let (mut state, message) = recurse(node, state, message);
            if let Message::IsPending = message {
                if let Message::IsPending = first_message {
                    state.rejected += 1;
                }
            }
            (state, message)
        },

        Element::Queue(no, max) => {
            // Propagate queued messages
            let message = if let Message::IsConsumed = message {
                if state.state[*no] > 0 {
                    state.state[*no] -= 1;
                    Message::IsPending
                } else {
                    Message::IsConsumed
                } 
            } else {
                Message::IsPending
            };

            let (mut state, message) = recurse(node, state, message);

            // Consume message if there is a free space
            match message {
                Message::IsPending if state.state[*no] < *max => {
                    state.state[*no] += 1;
                    (state, Message::IsConsumed)
                }
                Message::IsPending => (state, message),
                Message::IsConsumed => (state, message),
            }
        }
        Element::Retractor(no, chance) => {
            let (state, message) = match message {
                Message::IsPending if state.state[*no] == 1 => {
                    if cointoss < *chance { 
                        state.loaded[*no] += 1;
                        (state, Message::IsPending)
                    } else { 
                        state.consumed[*no] += 1;
                        state.loaded[*no] += 1;
                        (state, Message::IsConsumed)
                    } 
                },
                Message::IsPending => {
                    // Didn't have a message, now consumed one
                    state.loaded[*no] += 1;
                    state.state[*no] = 1;
                    (state, Message::IsConsumed)
                },
                Message::IsConsumed if state.state[*no] == 1 => {
                    if cointoss < *chance {
                        state.loaded[*no] += 1;
                        (state, Message::IsConsumed)
                    } else {
                        state.state[*no] = 0;
                        state.consumed[*no] += 1;
                        (state, Message::IsConsumed)
                    }
                }
                Message::IsConsumed => {
                    (state, Message::IsConsumed)
                },
            };
            recurse(node, state, message)
        }
    }
}

pub struct Simulation {
    pub states: Vec<([u8; 3], f32)>,

    pub chance_reject: f32,
    pub chance_block: f32,
    pub average_queue_num: f32,
    pub average_system_num: f32,
    pub average_queue_time: f32,
    pub average_system_time: f32,
    pub relative_throughput: f32,
    pub absolute_throughput: f32,
    pub average_channel_load_1: f32,
    pub average_channel_load_2: f32,
}


pub fn state_to_int(state: [u8; 3]) -> String {
    state
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}




pub fn simulate(ro: f32, qsize: u8, pi1: f32, pi2: f32) -> Simulation {
    let mut r = Random::default();
    const ITERATIONS: u64 = 500_000;

    let sys = var36(ro, qsize, pi1, pi2);
    let mut state = [0; 3];
    let mut state = State {
        state,

        loaded: [0; 3],
        consumed: [0; 3],
        rejected: 0,
    };


    let mut counters = HashMap::new();
    let mut whole_queue_size: f32 = 0.;
    let mut whole_system_size: f32 = 0.;

    for _ in 0..ITERATIONS {
        let counter = counters.entry(state.state).or_insert(0);
        *counter += 1;
        whole_queue_size += state.state[0] as f32;
        whole_system_size += state.state.iter().sum::<u8>() as f32;

        state = step(&sys, &mut r, state);
    }

    let states: Vec<_> = counters.into_iter().map(|(state, counter)| (state, counter as f32 / ITERATIONS as f32)).collect();

    let whole_consumed_1 = state.consumed[1] as f32;
    let whole_consumed_2 = state.consumed[2] as f32;


    let chance_reject: f32;
    let chance_block: f32;
    let average_queue_num: f32;
    let average_system_num: f32;
    let average_queue_time: f32;
    let average_system_time: f32;
    let relative_throughput: f32;
    let absolute_throughput: f32;
    let average_channel_load_1: f32;
    let average_channel_load_2: f32;
    
    chance_block = 0.;
    chance_reject = state.rejected as f32 / ITERATIONS as f32;

    let absolute_throughput_queue = whole_consumed_1 / ITERATIONS as f32;
    println!("{}",absolute_throughput_queue);
    absolute_throughput = (whole_consumed_1 + whole_consumed_2) / ITERATIONS as f32;


    relative_throughput = 1. - chance_reject;

    average_queue_num = 
        whole_queue_size / ITERATIONS as f32;
    average_system_num = 
        whole_system_size / ITERATIONS as f32;

    average_queue_time = average_queue_num / absolute_throughput_queue;
    average_system_time = average_system_num / absolute_throughput;
    
    average_channel_load_1 = state.loaded[1] as f32 / ITERATIONS as f32;
    average_channel_load_2 = state.loaded[2] as f32 / ITERATIONS as f32;

    Simulation {
        states,
        chance_reject,
        chance_block,
        average_queue_num,
        average_system_num,
        average_queue_time,
        average_system_time,
        relative_throughput,
        absolute_throughput,
        average_channel_load_1,
        average_channel_load_2,
    }

}

