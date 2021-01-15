mod arg_parsing;
mod simulation;

use crate::arg_parsing::{ProgArgLogLevel, ProgArgs};
use crate::simulation::{get_ms_as_str, SimTimeType, UPDATE_CYCLE_FREQ};
use log::info;

fn main() {
    let p_args = arg_parsing::parse_prog_args();
    setup_env_logger_from_prog_args_log_level(&p_args.log_level);
    print_info_about_prog_args(&p_args);

    let mut ss = simulation::init_state(&p_args);

    for event_num in 0..p_args.num_events_to_proc {
        println!("\n\n---------- Event #{}... ----------", event_num);
        simulation::run_sim_until_next_event(&mut ss);
    }

    println!(
        "Simulation finished running for {} events!",
        p_args.num_events_to_proc
    );
}

fn setup_env_logger_from_prog_args_log_level(log_level: &ProgArgLogLevel) {
    let log_env_var_str = match log_level {
        ProgArgLogLevel::Trace => "trace",
        ProgArgLogLevel::Debug => "debug",
        ProgArgLogLevel::Info => "info",
        ProgArgLogLevel::Warn => "warn",
        ProgArgLogLevel::Err => "error",
    };

    std::env::set_var("RUST_LOG", log_env_var_str);
    pretty_env_logger::init();
}

fn print_info_about_prog_args(p_args: &ProgArgs) {
    let lockstep_buffer_ms = p_args.l_buf_size as SimTimeType * UPDATE_CYCLE_FREQ;

    info!(
        "Lockstep buffer size: {} frames ({})",
        p_args.l_buf_size,
        get_ms_as_str(lockstep_buffer_ms)
    );
    info!(
        "Latency mean: {}",
        get_ms_as_str(p_args.lat_mean as SimTimeType)
    );
    info!(
        "Latency std: {}",
        get_ms_as_str(p_args.lat_std as SimTimeType)
    );
    info!(
        "Simulation length: {} events",
        p_args.num_events_to_proc as SimTimeType
    );
}
