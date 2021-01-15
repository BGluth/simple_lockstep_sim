use clap::{App, Arg};

const ARG_LOCKSTEP_BUF_SIZE_STR: &str = "lockstep_buffer_size";
const ARG_LAT_SIM_TYPE: &str = "latency_sim_type";

const DEFAULT_LOCKSTEP_BUFF_SIZE: &str = "3";

pub enum LatencySimType {
    CONSTANT = 0,
}

pub struct ProgArgs {
    pub l_buf_size: usize,
    pub lat_type: LatencySimType,
}

pub fn parse_prog_args() -> ProgArgs {
    let arg_matches = App::new("Simple Lockstep Simulator")
        .author("Brendan Gluth")
        .about("Simple tool to test some stuff with lockstep")
        .arg(
            Arg::new(ARG_LOCKSTEP_BUF_SIZE_STR)
                .required(false)
                .short('b')
                .case_insensitive(true)
                .long("lockstep_buffer_size")
                .default_value(DEFAULT_LOCKSTEP_BUFF_SIZE)
                .about("The lockstep buffer size"),
        )
        .arg(
            Arg::new(ARG_LAT_SIM_TYPE)
                .short('s')
                .long("sim_type")
                .value_name("LEVEL")
                .case_insensitive(true)
                .possible_values(&["c"])
                .default_value("c")
                .about("Sets the latency sim type"),
        )
        .get_matches();

    let l_buf_size = arg_matches
        .value_of(ARG_LOCKSTEP_BUF_SIZE_STR)
        .unwrap()
        .parse()
        .unwrap();
    let lat_type = match arg_matches.value_of(ARG_LAT_SIM_TYPE).unwrap() {
        "c" => LatencySimType::CONSTANT,
        _ => panic!("Got an invalid latency sim type (should not be possible!)"),
    };

    ProgArgs {
        l_buf_size,
        lat_type,
    }
}
