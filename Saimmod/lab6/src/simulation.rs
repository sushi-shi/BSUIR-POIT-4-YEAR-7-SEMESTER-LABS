use std::collections::HashMap;
use std::ops::Add;

use rand;

type Chance = f32;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
struct Timestamp(f32);

impl Add for Timestamp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let Timestamp(lhs) = self;
        let Timestamp(rhs) = rhs;
        Timestamp(lhs + rhs)
    }
}

#[derive(Copy, Clone)]
struct System {
    gen: (Chance, Timestamp),
    queue: Option<Message>,
    channel: (Chance, Option<(Timestamp, Message)>),

    rho: Chance,

    state: State,
}

#[derive(Debug, Default, Copy, Clone)]
struct State {
    generated_ord: usize,
    generated_pri: usize,

    processed_ord: usize,
    processed_pri: usize,

    dropped_ord: usize,
    dropped_pri: usize,
}

#[derive(Copy, Clone)]
struct Message {
    ty: MessageTy,
}

#[derive(Copy, Clone)]
enum MessageTy {
    Priority,
    Ordinary,
}

fn exp_dist(lambda: Chance) -> Timestamp {
    let r = rand::random::<f32>();
    Timestamp(-(1. / lambda) * f32::ln(r))
}

impl System {
    fn new(mu: Chance, lambda: Chance, rho: Chance) -> System {
        System {
            gen: (lambda, exp_dist(lambda)),
            queue: None,
            channel: (mu, None),

            rho,
            state: State::default(),
        }
    }

    fn next_time(&self) -> Timestamp {
        match self.channel.1 {
            None => self.gen.1,
            Some(x) => {
                if x.0 < self.gen.1 {
                    x.0
                } else {
                    self.gen.1
                }
            }
        }
    }

    fn process(&mut self) -> Option<()> {
        let message = self.channel.1?.1;
        match message.ty {
            MessageTy::Priority => self.state.processed_pri += 1,
            MessageTy::Ordinary => self.state.processed_ord += 1,
        }
        Some(())
    }

    fn state(&self) -> [usize; 2] {
        [
            match self.queue {
                None => 0,
                Some(Message {
                    ty: MessageTy::Priority,
                    ..
                }) => 1,
                Some(Message {
                    ty: MessageTy::Ordinary,
                    ..
                }) => 2,
            },
            match self.channel.1 {
                None => 0,
                Some((
                    _,
                    Message {
                        ty: MessageTy::Priority,
                        ..
                    },
                )) => 1,
                Some((
                    _,
                    Message {
                        ty: MessageTy::Ordinary,
                        ..
                    },
                )) => 2,
            },
        ]
    }

    fn generate(&mut self) -> Message {
        Message {
            ty: if rand::random::<f32>() > self.rho {
                self.state.generated_ord += 1;
                MessageTy::Ordinary
            } else {
                self.state.generated_pri += 1;
                MessageTy::Priority
            },
        }
    }

    // My variant is hardcoded into this, that's why it looks so spaghetty.
    // The better solution would be to use graphs with different types at nodes and traverse it
    // recursively each step (as I did in the third lab)
    fn step(mut self) -> Self {
        let gtime = self.gen.1;

        match self.channel.1 {
            Some((ctime, cmessage)) => {
                if ctime < gtime {
                    self.process();
                    match self.queue {
                        None => self.channel.1 = None,
                        Some(qmessage) => {
                            self.channel.1 = Some((ctime + exp_dist(self.channel.0), qmessage));
                            self.queue = None;
                        }
                    }
                } else {
                    let gmessage = self.generate();
                    match (gmessage.ty, self.queue) {
                        (MessageTy::Ordinary, Some(_)) => self.state.dropped_ord += 1, // dropped
                        (MessageTy::Ordinary, None) => self.queue = Some(gmessage),

                        (MessageTy::Priority, None) => match cmessage.ty {
                            MessageTy::Priority => self.queue = Some(gmessage),
                            MessageTy::Ordinary => {
                                self.queue = Some(cmessage);
                                self.channel.1 = Some((gtime + exp_dist(self.channel.0), gmessage));
                            }
                        },
                        (MessageTy::Priority, Some(qmessage)) => match (qmessage.ty, cmessage.ty) {
                            (MessageTy::Priority, MessageTy::Priority) => {
                                self.state.dropped_pri += 1
                            }
                            (MessageTy::Ordinary, MessageTy::Priority) => {
                                self.queue = Some(gmessage);
                                self.state.dropped_ord += 1
                            }
                            (MessageTy::Ordinary, MessageTy::Ordinary) => {
                                self.channel.1 = Some((gtime + exp_dist(self.channel.0), gmessage));
                                self.state.dropped_ord += 1
                            }
                            (MessageTy::Priority, MessageTy::Ordinary) => unreachable!(),
                        },
                    }
                    self.gen.1 = gtime + exp_dist(self.gen.0);
                }
            }
            None => {
                let gmessage = self.generate();
                self.channel.1 = Some((gtime + exp_dist(self.channel.0), gmessage));
                self.gen.1 = gtime + exp_dist(self.gen.0);
            }
        };
        self
    }
}

pub struct Simulation {
    pub states: Vec<([usize; 2], Chance)>,

    pub chance_reject_1: Chance,
    pub chance_reject_2: Chance,
    pub relative_throughput_1: Chance,
    pub relative_throughput_2: Chance,
}

pub fn pretty_state(state: [usize; 2]) -> String {
    state
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}

pub fn simulate(mu: Chance, lambda: Chance, rho: Chance) -> Simulation {
    const ITERATIONS: u64 = 500_000;
    let mut system = System::new(mu, lambda, rho);
    let mut states = HashMap::new();

    let mut time_start = 0.;
    let mut time_end;
    let mut time_diff;

    for _ in 0..ITERATIONS {
        time_end = system.next_time().0;
        time_diff = time_end - time_start;
        time_start = time_end;

        let state = system.state();

        system = system.step();

        let counter = states.entry(state).or_insert(0.);
        *counter += time_diff;
    }

    println!("{:#?}", system.state);
    println!("{:#?}", system.next_time());

    let time = system.next_time().0;
    let state = system.state;

    println!(
        "All source intensity {}",
        (system.state.generated_pri + system.state.generated_ord) as f32 / system.next_time().0
    );

    println!(
        "All Channel intensity {}",
        (system.state.processed_pri + system.state.processed_ord) as f32 / system.next_time().0
    );

    println!(
        "All Dropped intensity {}",
        (system.state.dropped_pri + system.state.dropped_ord) as f32 / system.next_time().0
    );

    println!(
        "Priority dropped intensity {}",
        (state.dropped_pri as f32 / time)
    );

    let pri_its = lambda * rho * (1. - states.get(&[1, 1]).unwrap_or_else(|| &0.) / time);
    println!(
        "Priority channel intensity\ntheory: {}, \npractice: {}\n",
        pri_its,
        (state.processed_pri as f32 / time)
    );

    let good = states.get(&[0, 2]).unwrap_or_else(|| &0.) + states.get(&[2, 2]).unwrap_or_else(|| &0.);
    let good = good / time;
    let ord_its = mu * good;
    println!(
        "Ordinary channel intensity\ntheory: {}, \npractice: {}\n",
        ord_its,
        (state.processed_ord as f32 / time)
    );

    let states: Vec<_> = states
        .into_iter()
        .map(|(state, counter)| (state, counter as f32 / system.next_time().0 as f32))
        .collect();

    let relative_throughput_1 = state.processed_pri as f32 / state.generated_pri as f32;
    let relative_throughput_2 = state.processed_ord as f32 / state.generated_ord as f32;

    let chance_reject_1 = state.dropped_pri as f32 / state.generated_pri as f32;
    let chance_reject_2 = state.dropped_ord as f32 / state.generated_ord as f32;

    Simulation {
        states,
        chance_reject_1,
        chance_reject_2,
        relative_throughput_1,
        relative_throughput_2,
    }
}
