use emotional_faces_recode::{parse_events, reconstruct_trials};

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
    let correct_trial_count = trials.iter().filter(|trial| trial.correct_response).count();
    std::fs::write(
        output_file,
        format!("{}, {}", input_file, correct_trial_count),
    )
    .expect("Failed to write file.");
}
