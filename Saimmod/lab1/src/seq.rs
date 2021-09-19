use nohash_hasher::IntMap;

pub const DEFAULT_ARGUMENTS: (&str, &str, &str) = ("1103515", "345345", "543543421");
pub const COLUMN_LENGTH: usize = 20;

pub struct InfiniteLemerSequence {
    a: u64,
    m: u64,
    r_curr: u64,
}


impl InfiniteLemerSequence {
    pub fn new(a: u64, r_0: u64, m: u64) -> Self {
        InfiniteLemerSequence { 
            a,
            m,
            r_curr: r_0 % m,
        }
    }
}

impl Iterator for InfiniteLemerSequence {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.r_curr;
        self.r_curr = (self.a * self.r_curr) % self.m;
        Some(res)
    }
}

#[derive(Default)]
pub struct LemerSequenceMeta {
    pub len: usize,
    pub period_len: usize,
    pub aperiod_len: usize,

    pub mean: f64,
    pub dispersion: f64,
    pub deviation: f64,
    pub indirect_ratio: f64,
    pub cols: Vec<f64>,
}


pub fn analyze_sequence(lemer: InfiniteLemerSequence, col_number: usize) -> LemerSequenceMeta {
    // Number of elements in each column
    let mut cols: Vec<u64> = vec![0; col_number];

    // IntMap is used for tracking already encountered elements
    // If we were to use Vec instead, we would get O(n^2), which is bad
    let mut state = IntMap::default();
    
    // But we do need Vec for analyzing and iterating through the sequence 
    // to calculate mean and deviation
    let mut seq = Vec::new();

    // Modulus
    let m = lemer.m;


    let mut len = 0;
    let mut period_len = 0;
    let mut aperiod_len = 0;

    let mut mean = 0.0;
    let mut dispersion = 0.0;
    let mut deviation = 0.0;
    let mut indirect_ratio = 0.0;

    for x in lemer { // Infinite sequence
        // FIXME: Instead of verifying whether we are finished for each element,
        // it is faster to batch lots of elements together and do it once for every HIGH_NUMBER
        // elements, but I just don't care enough to do it
        match state.get(&x) {  
            // New element in a sequence
            None => {
                let squashed = x as f64 / m as f64;

                let i = (squashed * col_number as f64) as usize;
                cols[i] += 1;

                let cur_idx = seq.len();
                state.insert(x, cur_idx);
                seq.push(squashed);

                mean += squashed;
            },

            // We've found a loop, initialize everything and break
            Some(old_idx) => {
                let period_start_idx = *old_idx;

                len = seq.len();
                period_len = len - period_start_idx;
                aperiod_len = period_start_idx;

                mean /= len as f64;
                dispersion = seq
                    .iter()
                    .map(|x| (x - mean).powf(2.0))
                    .sum::<f64>() 
                    / len as f64;
                deviation = dispersion.powf(0.5);
                let (even, odd): (Vec<_>, Vec<_>) = seq
                                  .iter().zip(0..)
                                  .partition(|(_, n)| n % 2 == 0);
                let even = even.into_iter().map(|(x, _)| *x);
                let odd = odd.into_iter().map(|(x, _)| *x);
                indirect_ratio = 2.0 * even
                    .zip(odd)
                    .filter(|(even, odd)| even.powf(2.0) + odd.powf(2.0) < 1.0)
                    .count() as f64
                    /  len as f64;
                    
                break;
            },
        }
    }
    let cols: Vec<f64> = cols.into_iter().map(|v| v as f64 / len as f64).collect();

    LemerSequenceMeta {
        len,
        period_len,
        aperiod_len,
        mean,
        dispersion,
        deviation,
        indirect_ratio,
        cols,
    }

}
