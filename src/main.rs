use emotional_faces_recode::{
    accuracy_percentage, parse_events, reaction_time_milliseconds, reconstruct_trials, Condition,
    Sex, Trial,
};

fn trials_matching(trials: &[Trial], condition: Condition, sex: Sex) -> Vec<Trial> {
    trials
        .iter()
        .cloned()
        .filter(|trial| trial.condition == condition && trial.sex == sex)
        .collect::<Vec<_>>()
}

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
            "{}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {}, {}, {}, {}, {}, {}, {}",
            input_file,
            accuracy_percentage(&trials),
            accuracy_percentage(&trials_matching(&trials, Condition::Angry, Sex::Male)),
            accuracy_percentage(&trials_matching(&trials, Condition::Happy, Sex::Male)),
            accuracy_percentage(&trials_matching(&trials, Condition::Neutral, Sex::Male)),
            accuracy_percentage(&trials_matching(&trials, Condition::Angry, Sex::Female)),
            accuracy_percentage(&trials_matching(&trials, Condition::Happy, Sex::Female)),
            accuracy_percentage(&trials_matching(&trials, Condition::Neutral, Sex::Female)),
            reaction_time_milliseconds(&trials),
            reaction_time_milliseconds(&trials_matching(&trials, Condition::Angry, Sex::Male)),
            reaction_time_milliseconds(&trials_matching(&trials, Condition::Happy, Sex::Male)),
            reaction_time_milliseconds(&trials_matching(&trials, Condition::Neutral, Sex::Male)),
            reaction_time_milliseconds(&trials_matching(&trials, Condition::Angry, Sex::Female)),
            reaction_time_milliseconds(&trials_matching(&trials, Condition::Happy, Sex::Female)),
            reaction_time_milliseconds(&trials_matching(&trials, Condition::Neutral, Sex::Female)),
        ),
    )
    .expect("Failed to write file.");
}
