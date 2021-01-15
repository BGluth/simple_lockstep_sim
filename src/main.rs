mod arg_parsing;
mod simulation;

use crate::arg_parsing::ProgArgLogLevel;

fn main() {
    let prog_args = arg_parsing::parse_prog_args();
    setup_env_logger_from_prog_args_log_level(&prog_args.log_level);

    let mut ss = simulation::init_state(&prog_args);

    for event_num in 0..prog_args.num_events_to_proc {
        println!("\n\n---------- Event #{}... ----------", event_num);
        simulation::run_sim_until_next_event(&mut ss);
    }
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
