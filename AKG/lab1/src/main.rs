
mod drawing;
mod transformations;
mod parser;
mod types;
mod opengl;

use parser::parse_obj_file;
use types::{
    State,
    CameraPosition,
};

fn fail_with(progname: &str, msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    fail_usage(progname);
}

fn fail_usage(progname: &str) -> ! {
    eprintln!("USAGE: {} PATH", progname);
    std::process::exit(1);
}

fn main() {
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

    let m_contents = std::fs::read_to_string(path);
    let contents = match m_contents {
        Err(e) => fail_with(progname, &e.to_string()),
        Ok(contents) => contents,
    };

    let m_obj_file = parse_obj_file(&contents);
    let obj_file = match m_obj_file {
        Err(e) => fail_with(progname, &e.to_string()),
        Ok((_, obj_file)) => obj_file,
    };

    let state = State {
        obj_file,
        camera: CameraPosition::default(),
    };

    crate::opengl::run(state);
}
