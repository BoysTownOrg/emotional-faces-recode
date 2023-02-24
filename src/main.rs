use emotional_faces_recode::{parse_events, reconstruct_trials};

fn main() {
    if let Some(file_path) = std::env::args().nth(1) {
        let contents = std::fs::read_to_string(file_path).unwrap();
        let events = parse_events(&contents);
        let trials = reconstruct_trials(events);
        println!("Trial count: {}", trials.len());
    }
}
