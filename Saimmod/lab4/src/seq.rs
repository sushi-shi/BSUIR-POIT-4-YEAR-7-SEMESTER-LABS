const DEFAULT_ARGUMENTS_2: (u64, u64, u64) = (1103515245, 345345, 543543422);

pub struct LemerSequence {
    a: u64,
    m: u64,
    r_curr: u64,
}


impl LemerSequence {
    pub fn new(a: u64, r_0: u64, m: u64) -> Self {
        LemerSequence { 
            a,
            m,
            r_curr: r_0 % m,
        }
    }
}

impl Default for LemerSequence {
    fn default() -> Self {
        LemerSequence::new(
            DEFAULT_ARGUMENTS_2.0,
            DEFAULT_ARGUMENTS_2.1,
            DEFAULT_ARGUMENTS_2.2,
        )
    }
}

impl Iterator for LemerSequence {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.r_curr;
        self.r_curr = (self.a * self.r_curr) % self.m;
        Some(res)
    }
}

pub struct Random {
    lemer: LemerSequence
}

impl Random {
    pub fn random(&mut self) -> f32 {
        self.next().unwrap()
    }
}

impl Default for Random {
    fn default() -> Self {
        Random {
            lemer: LemerSequence::default(),
        }
    }

}

impl Iterator for Random {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.lemer.next().map(|v| v as f32 / self.lemer.m as f32)
    }
}
