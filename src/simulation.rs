use crate::arg_parsing::ProgArgs;

use log::info;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::fmt;

use rand_distr::{Distribution, Normal};

type ClientID = usize;

const FPS: usize = 60;
pub const UPDATE_CYCLE_FREQ: usize = 1000 / FPS;

pub struct SimState {
    ws: WorldState,
    clients: Vec<Client>,
}

struct WorldState {
    ms_elapsed: usize,
    upcoming_events: BinaryHeap<Reverse<UpcomingEvent>>,
    ls: LatencyState,
}

impl WorldState {
    fn push_event(&mut self, event: UpcomingEvent) {
        self.upcoming_events.push(Reverse(event));
    }

    fn get_next_event(&mut self) -> UpcomingEvent {
        let reversed_event = self.upcoming_events.pop().unwrap();
        reversed_event.0
    }
}

struct LatencyState {
    lat_dist: Normal<f32>,
}

impl LatencyState {
    fn new(mean_lat: usize, std_lat: usize) -> LatencyState {
        LatencyState {
            lat_dist: Normal::new(mean_lat as f32, std_lat as f32).unwrap(),
        }
    }

    fn gen_delay_of_msg(&mut self) -> usize {
        self.lat_dist.sample(&mut rand::thread_rng()) as usize
    }
}

struct UpcomingEvent {
    event_trigger_time: usize,
    e_type: EventType,
}

impl Ord for UpcomingEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.event_trigger_time.cmp(&other.event_trigger_time)
    }
}

impl PartialOrd for UpcomingEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.event_trigger_time
            .partial_cmp(&other.event_trigger_time)
    }
}

impl PartialEq for UpcomingEvent {
    fn eq(&self, other: &Self) -> bool {
        self.event_trigger_time == other.event_trigger_time
    }
}

impl Eq for UpcomingEvent {}

enum EventType {
    NextUpdateCycle(ClientID),
    MessageArrival(Message),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::NextUpdateCycle(client_id) => {
                write!(f, "Update cycle for client {}", *client_id)
            }
            EventType::MessageArrival(msg) => write!(
                f,
                "Message arrived for client {} from client {} for update cycle {}",
                msg.dest_client, msg.sending_client, msg.target_update_cycle
            ),
        }
    }
}

struct Client {
    client_id: ClientID,
    curr_update_cycle: usize,
    lockstep_buffer_size: usize,
    update_cycles_waiting_on: Vec<WaitingClientUpdateCycle>,
    next_update_cycle_that_we_are_waiting_on: usize,
    is_stalled: bool,
}

impl Client {
    fn new(client_id: ClientID, l_buf_size: usize) -> Client {
        Client {
            client_id,
            curr_update_cycle: 0,
            lockstep_buffer_size: l_buf_size,
            update_cycles_waiting_on: Vec::new(),
            next_update_cycle_that_we_are_waiting_on: 0,
            is_stalled: false,
        }
    }

    fn add_cycle_to_waiting_on(&mut self, update_cycle: usize, num_clients: usize) {
        let our_client_id = self.client_id;

        for client_id in get_client_ids_that_are_not_ours(num_clients, our_client_id) {
            self.update_cycles_waiting_on
                .push(WaitingClientUpdateCycle {
                    from_client: client_id,
                    update_cycle,
                });
        }
    }
}

struct WaitingClientUpdateCycle {
    from_client: ClientID,
    update_cycle: usize,
}

struct Message {
    target_update_cycle: usize,
    sending_client: ClientID,
    dest_client: ClientID,
}

impl Message {
    fn new(target_update_cycle: usize, sending_client: ClientID, dest_client: ClientID) -> Message {
        Message {
            target_update_cycle,
            sending_client,
            dest_client,
        }
    }
}

pub fn init_state(p_args: &ProgArgs) -> SimState {
    let ws = WorldState {
        ms_elapsed: 0,
        upcoming_events: BinaryHeap::new(),
        ls: LatencyState::new(p_args.lat_mean, p_args.lat_std),
    };

    let mut clients = Vec::new();
    clients.push(Client::new(0, p_args.l_buf_size));
    clients.push(Client::new(1, p_args.l_buf_size));

    let mut ss = SimState { ws, clients };

    create_initial_client_input_frames(&mut ss, p_args.l_buf_size);
    create_initial_client_update_cycle_events(&mut ss);

    ss
}

pub fn run_sim_until_next_event(ss: &mut SimState) {
    let next_event = ss.ws.get_next_event();
    ss.ws.ms_elapsed = next_event.event_trigger_time;

    println!(
        "Event: {} (sim time: {}ms)",
        next_event.e_type, ss.ws.ms_elapsed
    );

    match next_event.e_type {
        EventType::NextUpdateCycle(client_id) => handle_update_cycle_event(ss, client_id),
        EventType::MessageArrival(msg) => handle_msg_received_event(ss, msg),
    };
}

fn create_initial_client_input_frames(ss: &mut SimState, l_buf_size: usize) {
    // Get a sane start state by getting clients to send initial input state to each other.
    let num_input_frames_to_create = l_buf_size;
    let num_clients = ss.clients.len();

    info!("Creating initial input cycle update message to initialize the simulation...");
    for i in 0..num_input_frames_to_create {
        for sending_client_id in 0..ss.clients.len() {
            for receiving_client_id in
                get_client_ids_that_are_not_ours(ss.clients.len(), sending_client_id)
            {
                let target_frame = i;
                let msg = Message::new(target_frame, sending_client_id, receiving_client_id);
                send_msg(msg, &mut ss.ws);
            }

            ss.clients[sending_client_id].add_cycle_to_waiting_on(i, num_clients);
        }
    }
}

fn create_initial_client_update_cycle_events(ss: &mut SimState) {
    for client_id in 0..ss.clients.len() {
        add_event_for_next_update_cycle_start(&mut ss.ws, client_id, 0);
    }
}

fn send_msg(msg: Message, ws: &mut WorldState) {
    let delay_of_msg = ws.ls.gen_delay_of_msg();

    info!(
        "Sending update to client {} from client {} for cycle {} with a delay of {}ms...",
        msg.dest_client, msg.sending_client, msg.target_update_cycle, delay_of_msg
    );

    let upcoming_event = UpcomingEvent {
        event_trigger_time: ws.ms_elapsed + delay_of_msg,
        e_type: EventType::MessageArrival(msg),
    };

    ws.push_event(upcoming_event);
}

fn add_event_for_next_update_cycle_start(
    ws: &mut WorldState,
    client_id: ClientID,
    time_of_next_update_cycle: usize,
) {
    info!(
        "Scheduling next update cycle for client {} at {}ms...",
        client_id, time_of_next_update_cycle
    );

    let update_event = UpcomingEvent {
        event_trigger_time: time_of_next_update_cycle,
        e_type: EventType::NextUpdateCycle(client_id),
    };

    ws.push_event(update_event);
}

fn handle_update_cycle_event(ss: &mut SimState, client_id: ClientID) {
    let num_clients = ss.clients.len();
    let curr_sim_time = ss.ws.ms_elapsed;

    let client = &mut ss.clients[client_id];
    println!(
        "Update cycle {} just completed for client {}",
        client.curr_update_cycle, client_id
    );

    if client.curr_update_cycle == client.next_update_cycle_that_we_are_waiting_on {
        // Stall!
        client.is_stalled = true;
        println!("Client {} has stalled!", client_id);
        return;
    }
    client.curr_update_cycle += 1;

    let msg_target_cycle = client.curr_update_cycle + client.lockstep_buffer_size;
    send_update_cycle_info_out_to_other_clients(
        &mut ss.ws,
        num_clients,
        client_id,
        msg_target_cycle,
    );
    client.add_cycle_to_waiting_on(msg_target_cycle, num_clients);

    add_event_for_next_update_cycle_start(&mut ss.ws, client_id, curr_sim_time + UPDATE_CYCLE_FREQ);
}

fn handle_msg_received_event(ss: &mut SimState, msg: Message) {
    process_recv_update_cycle_info_for_client(ss, msg.dest_client, msg);
}

fn send_update_cycle_info_out_to_other_clients(
    ws: &mut WorldState,
    num_clients: usize,
    client_id: ClientID,
    target_update_cycle: usize,
) {
    for other_client_id in get_client_ids_that_are_not_ours(num_clients, client_id) {
        let msg = Message::new(target_update_cycle, client_id, other_client_id);
        send_msg(msg, ws)
    }
}

fn process_recv_update_cycle_info_for_client(ss: &mut SimState, client_id: ClientID, msg: Message) {
    let curr_sim_time = ss.ws.ms_elapsed;

    let client = &mut ss.clients[client_id];

    let idx_of_waiting_msg = client
        .update_cycles_waiting_on
        .iter()
        .position(|x| {
            x.from_client == msg.sending_client && x.update_cycle == msg.target_update_cycle
        })
        .unwrap();

    client
        .update_cycles_waiting_on
        .swap_remove(idx_of_waiting_msg);

    while !client.update_cycles_waiting_on.is_empty()
        && client
            .update_cycles_waiting_on
            .iter()
            .all(|x| x.update_cycle != client.next_update_cycle_that_we_are_waiting_on)
    {
        println!(
            "Client {} just received all other pending client info for frame {}.",
            client_id, client.next_update_cycle_that_we_are_waiting_on
        );

        if client.is_stalled {
            add_event_for_next_update_cycle_start(&mut ss.ws, client_id, curr_sim_time); // Schedule another update cycle event immediately
            client.is_stalled = false;
        }

        client.next_update_cycle_that_we_are_waiting_on += 1;
    }
}

fn get_client_ids_that_are_not_ours(
    num_clients: usize,
    our_client_id: ClientID,
) -> impl Iterator<Item = usize> {
    (0..num_clients).filter(move |id| *id != our_client_id)
}
