use clap::{App, Arg, ArgMatches};

const ARG_LOCKSTEP_BUF_SIZE_STR: &str = "lockstep_buffer_size";
const ARG_LAT_MEAN: &str = "latency_mean";
const ARG_LAT_STD: &str = "latency_std";
const ARG_NUM_EVENTS_TO_PROC: &str = "num_events_to_proc";
const ARG_NAME_STR_VERBOSITY: &str = "verbosity";

const ARG_VAL_STR_VERB_TRACE: &str = "trace";
const ARG_VAL_STR_VERB_DEBUG: &str = "debug";
const ARG_STR_VAL_VERB_INFO: &str = "info";
const ARG_STR_VAL_VERB_WARN: &str = "warn";
const ARG_STR_VAL_VERB_ERROR: &str = "error";

const DEFAULT_LOCKSTEP_BUFF_SIZE: &str = "3";

#[derive(Debug)]
pub enum ProgArgLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Err,
}

pub struct ProgArgs {
    pub l_buf_size: usize,
    pub lat_mean: usize,
    pub lat_std: usize,
    pub num_events_to_proc: usize,
    pub log_level: ProgArgLogLevel,
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
        .arg(Arg::new(ARG_NAME_STR_VERBOSITY)
            .short('v')
            .long(ARG_NAME_STR_VERBOSITY)
            .value_name("LEVEL")
            .possible_values(&[ARG_VAL_STR_VERB_TRACE, ARG_VAL_STR_VERB_DEBUG, ARG_STR_VAL_VERB_INFO, ARG_STR_VAL_VERB_WARN, ARG_STR_VAL_VERB_ERROR])
            .case_insensitive(true)
            .default_value(ARG_STR_VAL_VERB_INFO)
            .about("Sets the verbosity level of program output info, where \"trace\" is the most verbose while \"error\" is the least"))
        .get_matches();

    let log_level_str = arg_matches
        .value_of(ARG_NAME_STR_VERBOSITY)
        .unwrap()
        .to_lowercase();

    let log_level = match log_level_str.as_str() {
        ARG_VAL_STR_VERB_TRACE => ProgArgLogLevel::Trace,
        ARG_VAL_STR_VERB_DEBUG => ProgArgLogLevel::Debug,
        ARG_STR_VAL_VERB_INFO => ProgArgLogLevel::Info,
        ARG_STR_VAL_VERB_WARN => ProgArgLogLevel::Warn,
        ARG_STR_VAL_VERB_ERROR => ProgArgLogLevel::Err,
        _ => panic!(
            "Somehow got a not valid verbosity log level ({}). This should not be possible!",
            log_level_str
        ),
    };

    ProgArgs {
        l_buf_size: get_usize_val_of_prog_arg(&arg_matches, ARG_LOCKSTEP_BUF_SIZE_STR),
        lat_mean: get_usize_val_of_prog_arg(&arg_matches, ARG_LAT_MEAN),
        lat_std: get_usize_val_of_prog_arg(&arg_matches, ARG_LAT_STD),
        num_events_to_proc: get_usize_val_of_prog_arg(&arg_matches, ARG_NUM_EVENTS_TO_PROC),
        log_level,
    }
}

fn get_usize_val_of_prog_arg(arg_matches: &ArgMatches, arg_str: &str) -> usize {
    arg_matches.value_of(arg_str).unwrap().parse().unwrap()
}
