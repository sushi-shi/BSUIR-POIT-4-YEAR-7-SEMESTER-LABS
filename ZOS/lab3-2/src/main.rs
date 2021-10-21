use hound;
use rustfft::{num_complex::Complex, FftPlanner};

fn fail_with(progname: &str, msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    fail_usage(progname);
}

fn fail_usage(progname: &str) -> ! {
    eprintln!("USAGE: {} PATH", progname);
    std::process::exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    let m_progname = args.next();
    let progname = &match m_progname {
        None => fail_with("program-name", "Couldn't get name of the program"),
        Some(progname) => progname,
    };

    let m_path = args.next();
    let path = match m_path {
        None => fail_with(progname, "No arguments provided"),
        Some(path) => path,
    };

    let reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let len = reader.len() as usize;
    let sample_rate = 5 * spec.sample_rate as usize;

    let mut samples = reader
        .into_samples::<i16>()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|i| Complex::new(i as f32, 0f32))
        .take(len - len % sample_rate)
        .collect::<Vec<_>>();

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(sample_rate);
    fft.process(&mut samples);

    let mut samples = samples
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let i = i % sample_rate;
            const LOW_BOUND: usize = 500;
            const HIGH_BOUND: usize = 5000;
            if i > LOW_BOUND && i < HIGH_BOUND
                || i < sample_rate - LOW_BOUND && i > sample_rate - HIGH_BOUND
            {
                // if true {
                c.unscale(sample_rate as f32 / 2.)
            } else {
                Complex::new(0f32, 0f32)
            }
        })
        .collect::<Vec<_>>();

    let fft = planner.plan_fft_inverse(sample_rate);
    fft.process(&mut samples);

    let samples = samples
        .into_iter()
        .map(|c| (c.re / 2.) as i16)
        .collect::<Vec<_>>();

    let mut writer = hound::WavWriter::create("result.wav", spec)?;
    for sample in samples {
        writer.write_sample(sample)?;
    }

    Ok(())
}
