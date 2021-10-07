use petgraph::dot::Dot;
use petgraph::graph;
use petgraph::graph::{Graph, NodeIndex};

use ndarray::prelude::*;
use ndarray_linalg::Solve;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

/// There are two invariants:
/// 1. Entry is a Generator
/// 2. Graph doesn't contain loops
struct System {
    graph: graph::Graph<Element, ()>,
    entry: graph::NodeIndex,
    state_size: usize,
    ro: f32,
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
type StateTransitions = graph::Graph<State, Transition>;
type Chance = f32;
type State = Vec<u8>;

type Transition = (Chance, Vec<String>);

fn var36() -> System {
    const RO: f32 = 0.3;
    const QSIZE: u8 = 2;
    const PI1: f32 = 0.8;
    const PI2: f32 = 0.75;

    let mut graph = graph::Graph::new();
    let gen = graph.add_node(Element::Generator(RO));
    let que = graph.add_node(Element::Queue(0, QSIZE));
    let ret1 = graph.add_node(Element::Retractor(1, PI1));
    let ret2 = graph.add_node(Element::Retractor(2, PI2));

    let _ = graph.add_edge(gen, que, ());
    let _ = graph.add_edge(gen, ret2, ());
    let _ = graph.add_edge(que, ret1, ());

    System {
        graph,
        entry: gen,
        state_size: 3,
        ro: RO,
        pi1: PI1,
        pi2: PI2,
    }
}

// frigging list monad..
fn step(system: &System, state: State) -> Vec<(State, Transition)> {
    let graph = &system.graph;
    let entry = system.entry;

    let mut states = Vec::new();

    for message in &[Message::IsPending, Message::IsConsumed] {
        step_rec(graph, entry, state.clone(), *message)
            .into_iter()
            .map(|(s, t, _)| (s, t))
            .for_each(|x| states.push(x));
    }

    states
}

const RHO: &str = "ρ";
const INV_RHO: &str = "(1 - ρ)";

fn concat<T>(x: Vec<T>, y: Vec<T>) -> Vec<T> {
    x.into_iter().chain(y.into_iter()).collect()
}

/// We need to return a list of
///     1. A chance we get into this state
///     2. Whether given Message was consumed or not
///     3. State itself
///     4. String to append to our state
///
fn step_rec(
    graph: &Graph<Element, ()>,
    node: NodeIndex,
    state: State,
    message: Message,
) -> Vec<(State, Transition, Message)> {
    let mut state = state;

    if let Some(Element::Generator(ro)) = graph.node_weight(node) {
        let mut states = match message {
            Message::IsPending => vec![(state, (1.0 - *ro, vec![INV_RHO.to_string()]), message)],
            Message::IsConsumed => vec![(state, (*ro, vec![RHO.to_string()]), message)],
        };

        let mut nodes = graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .map(|idx| (idx, graph.node_weight(idx).unwrap()))
            .collect::<Vec<_>>();
        nodes.sort_by_key(|x| x.1.no());

        for node in nodes.iter().map(|x| x.0) {
            let mut tmp_states = Vec::new();
            for (state, (chance, label), message) in states {
                tmp_states.append(
                    &mut step_rec(graph, node, state, message)
                        .into_iter()
                        .map(|(s, (c, l), m)| (s, (c * chance, concat(label.clone(), l)), m))
                        .collect(),
                );
            }
            states = tmp_states;
        }
        return states;
    }

    if let Some(Element::Queue(no, max)) = graph.node_weight(node) {
        // TODO: General one with a recursion
        // Don't forget to sort by no
        // queue can have only one connected graph and in our case it is a single retractor
        let retractor = graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .next()
            .unwrap();

        let message = if let Message::IsConsumed = message {
            if state[*no] > 0 {
                state[*no] -= 1;
                Message::IsPending
            } else {
                Message::IsConsumed
            }
        } else {
            Message::IsPending
        };

        let rss = step_rec(graph, retractor, state, message);

        return rss
            .into_iter()
            .map(|(mut state, tr, message)| match message {
                Message::IsPending if state[*no] < *max => {
                    state[*no] += 1;
                    (state, tr, Message::IsConsumed)
                }
                Message::IsPending => (state, tr, message),
                Message::IsConsumed => (state, tr, message),
            })
            .collect();
    }

    if let Some(Element::Retractor(no, chance)) = graph.node_weight(node) {
        // TODO: General one with a recursion
        return match message {
            Message::IsPending if state[*no] == 1 => vec![
                (
                    state.clone(),
                    (*chance, vec![format!("π{}", no.to_string())]),
                    Message::IsPending,
                ),
                (
                    state,
                    (1.0 - *chance, vec![format!("(1 - π{})", no.to_string())]),
                    Message::IsConsumed,
                ),
            ],
            Message::IsPending => {
                state[*no] = 1;
                vec![(state, (1.0, Vec::new()), Message::IsConsumed)]
            }
            Message::IsConsumed if state[*no] == 1 => {
                let mut state_ = state.clone();
                state_[*no] = 0;
                vec![
                    (
                        state,
                        (*chance, vec![format!("π{}", no.to_string())]),
                        Message::IsConsumed,
                    ),
                    (
                        state_,
                        (1.0 - *chance, vec![format!("(1 - π{})", no.to_string())]),
                        Message::IsConsumed,
                    ),
                ]
            }
            Message::IsConsumed => {
                vec![(state, (1.0, Vec::new()), Message::IsConsumed)]
            }
        };
    }
    unreachable!()
}

fn state_transitions(system: &System) -> StateTransitions {
    let mut stts = StateTransitions::new();

    let init_state = vec![0; system.state_size];
    let mut new = vec![init_state.clone()];
    let mut stack = Vec::new();

    let mut visited = HashMap::new();
    let init_state_idx = stts.add_node(init_state.clone());
    let _none = visited.insert(init_state, init_state_idx);

    loop {
        if new.is_empty() {
            break;
        }

        for from_state in new {
            let from_state_idx = *visited
                .get(&from_state)
                .expect("All new nodes are already in a hashmap");

            for (to_state, transition) in step(system, from_state.clone()) {
                let to_state_idx = match visited.get(&to_state) {
                    None => {
                        // a new
                        // Has not been visited, let's visit it later
                        stack.push(to_state.clone());
                        // Insert into our graph
                        let to_state_idx = stts.add_node(to_state.clone());
                        // Mark visited
                        visited.insert(to_state.clone(), to_state_idx);

                        to_state_idx
                    }
                    Some(to_state_idx) => *to_state_idx,
                };
                let _node_idx = stts.add_edge(from_state_idx, to_state_idx, transition);
            }
        }
        new = stack;
        stack = Vec::new();
    }

    stts
}

fn state_to_int(state: &State) -> String {
    state
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn main() -> Result<(), Box<dyn Error>> {
    let sys = var36();
    let stts = state_transitions(&sys);
    let calcy_stts = stts.clone();
    let pretty_stts = stts.map(|_, x| state_to_int(x), |_, (_, labels)| labels.join("*"));

    fs::write("./graph.draw", Dot::new(&pretty_stts).to_string())?;

    let mut equations = Array2::zeros((calcy_stts.node_count(), calcy_stts.node_count()));
    let mut solutions = Array::zeros(calcy_stts.node_count());
    let mut names = Vec::new();
    println!("Equations:");
    for state_idx in calcy_stts.node_indices() {
        let state = calcy_stts.node_weight(state_idx).unwrap();
        let state = state_to_int(state);
        names.push(state.clone());

        let mut stack = Vec::new();
        for neighbor_idx in calcy_stts
            .neighbors_directed(state_idx, petgraph::Direction::Incoming)
            .collect::<HashSet<_>>()
        {
            let neighbor = calcy_stts.node_weight(neighbor_idx).unwrap();
            let neighbor = state_to_int(neighbor);

            let mut chance = 0.;
            for transition in calcy_stts.edges_connecting(neighbor_idx, state_idx) {
                stack.push(format!(
                    "({}) * P{}",
                    transition.weight().1.join(" * "),
                    neighbor
                ));
                chance += transition.weight().0;
            }
            equations[[state_idx.index(), neighbor_idx.index()]] = chance;
        }
        equations[[state_idx.index(), state_idx.index()]] -= 1.;
        println!("P{} = {}", state, stack.join(" + "));
    }

    solutions[0] = 1.;
    for i in 0..calcy_stts.node_count() {
        equations[[0, i]] = 1.; // overwrite one equation
    }

    let chances = equations.solve_into(solutions).unwrap();
    let states: Vec<_> = calcy_stts
        .node_indices()
        .map(|x| calcy_stts.node_weight(x).unwrap().clone())
        .zip(chances.iter().copied())
        .collect();

    println!("\nChances:");
    let mut names = names.into_iter().zip(chances).collect::<Vec<_>>();
    names.sort_by_key(|x| x.0.clone());
    names
        .into_iter()
        .for_each(|(name, res)| println!("P{} = {}", name, res));

    let chance_reject: f32;
    let chance_block: f32;
    let average_queue_num: f32;
    let average_system_num: f32;
    let average_queue_time: f32;
    let average_system_time: f32;
    let relative_throughput: f32;
    let average_channel_load_1: f32;
    let average_channel_load_2: f32;

    let filter_sum = |f: Box<dyn Fn(&State) -> _>| {
        states
            .iter()
            .filter(|(state, _)| f(state))
            .map(|x| x.1)
            .sum::<f32>()
    };

    chance_block = 0.;

    chance_reject = filter_sum(Box::new(|state| {
        state[1] == 1 && state[2] == 1 && state[0] == 2
    })) * sys.pi1
        * sys.pi2
        * (1. - sys.ro);

    let when_1_1 = filter_sum(Box::new(|state| state[1] == 1));
    let when_1_2 = filter_sum(Box::new(|state| state[2] == 1));

    // Среднее количество за такт
    let absolute_throughput_queue = when_1_1 * (1. - sys.pi1);
    let absolute_throughput_down = when_1_2 * (1. - sys.pi2);

    relative_throughput = 1. - chance_reject;

    average_queue_num = filter_sum(Box::new(|state| state[0] == 1))
        + 2. * filter_sum(Box::new(|state| state[0] == 2));
    average_system_num = states
        .iter()
        .map(|(state, c)| (state[0] + state[1] + state[2], c))
        .map(|(n, c)| n as f32 * c)
        .sum::<f32>();

    average_queue_time = average_queue_num / absolute_throughput_queue;
    average_system_time =
        average_system_num / (absolute_throughput_queue + absolute_throughput_down);

    average_channel_load_1 = filter_sum(Box::new(|state| state[1] == 1));
    average_channel_load_2 = filter_sum(Box::new(|state| state[2] == 1));

    println!("\nCharacteristics:");

    println!("P_от: {}", chance_reject);
    println!("P_bl: {}", chance_block);
    println!("L_оч: {}", average_queue_num);
    println!("L_c : {}", average_system_num);
    println!("Q   : {}", relative_throughput);
    println!(
        "A   : {}",
        (absolute_throughput_queue + absolute_throughput_down)
    );
    println!("W_оч: {}", average_queue_time);
    println!("W_c : {}", average_system_time);
    println!("K_1 : {}", average_channel_load_1);
    println!("K_2 : {}", average_channel_load_2);

    Ok(())
}
