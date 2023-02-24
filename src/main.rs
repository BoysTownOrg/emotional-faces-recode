use emotional_faces_recode::{
    accuracy_percentage, parse_events, reaction_time_milliseconds, reconstruct_trials,
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = std::fs::read_to_string(input_file).unwrap();
    let events = parse_events(&contents);
    let trials = reconstruct_trials(events);
    if trials.len() != 240 {
        panic!("Unexpected number of trials: {}", trials.len());
    }
    std::fs::write(
        output_file,
        format!(
            "{}, {:.2}, {}",
            input_file,
            accuracy_percentage(&trials),
            reaction_time_milliseconds(&trials)
        ),
    )
    .expect("Failed to write file.");
}
