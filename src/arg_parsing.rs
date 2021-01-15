use clap::{App, Arg, ArgMatches};

const ARG_LOCKSTEP_BUF_SIZE_STR: &str = "lockstep_buffer_size";
const ARG_LAT_MEAN: &str = "latency_mean";
const ARG_LAT_STD: &str = "latency_std";
const ARG_NUM_EVENTS_TO_PROC: &str = "num_events_to_proc";

const DEFAULT_LOCKSTEP_BUFF_SIZE: &str = "3";

pub struct ProgArgs {
    pub l_buf_size: usize,
    pub lat_mean: usize,
    pub lat_std: usize,
    pub num_events_to_proc: usize,
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
            Arg::new(ARG_LAT_MEAN)
                .short('m')
                .long("lat_mean")
                .default_value("50")
                .about("The mean latency of all packets"),
        )
        .arg(
            Arg::new(ARG_LAT_STD)
                .short('d')
                .long("lat_std")
                .default_value("5")
                .about("The standard deviation of the latency for msgs generated"),
        )
        .arg(
            Arg::new(ARG_NUM_EVENTS_TO_PROC)
                .short('n')
                .long("num_events")
                .default_value("100")
                .about("The number of events to process in the simulation"),
        )
        .get_matches();

    ProgArgs {
        l_buf_size: get_usize_val_of_prog_arg(&arg_matches, ARG_LOCKSTEP_BUF_SIZE_STR),
        lat_mean: get_usize_val_of_prog_arg(&arg_matches, ARG_LAT_MEAN),
        lat_std: get_usize_val_of_prog_arg(&arg_matches, ARG_LAT_STD),
        num_events_to_proc: get_usize_val_of_prog_arg(&arg_matches, ARG_NUM_EVENTS_TO_PROC),
    }
}

fn get_usize_val_of_prog_arg(arg_matches: &ArgMatches, arg_str: &str) -> usize {
    arg_matches.value_of(arg_str).unwrap().parse().unwrap()
}
