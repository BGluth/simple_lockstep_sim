mod arg_parsing;
mod simulation;

fn main() {
    let prog_args = arg_parsing::parse_prog_args();

    let mut ss = simulation::init_state(&prog_args);

    loop {
        simulation::run_sim_until_next_event(&mut ss);
    }
}
