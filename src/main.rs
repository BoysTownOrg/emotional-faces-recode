use emotional_faces_recode::{
    accuracy_percentage, parse_events, reaction_time_milliseconds, reconstruct_trials, Condition,
    Sex, Trial,
};
use std::io::Write;

fn trials_matching(trials: &[Trial], condition: Condition, sex: Sex) -> Vec<Trial> {
    trials
        .iter()
        .cloned()
        .filter(|trial| trial.condition == condition && trial.sex == sex)
        .collect::<Vec<_>>()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_file_path = &args[1];
    let output_file_path = &args[2];
    let contents = std::fs::read_to_string(input_file_path).unwrap();
    let events = parse_events(&contents);
    let trials = reconstruct_trials(events);
    if trials.len() != 240 {
        panic!("Unexpected number of trials: {}", trials.len());
    }
    let mut output_file = match std::fs::File::create(&output_file_path) {
        Err(why) => panic!("couldn't create {}: {}", output_file_path, why),
        Ok(file) => file,
    };
    writeln!(
        output_file,
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        "File",
        "All Accuracy (%)",
        "Angry Male Accuracy (%)",
        "Happy Male Accuracy (%)",
        "Neutral Male Accuracy (%)",
        "Angry Female Accuracy (%)",
        "Happy Female Accuracy (%)",
        "Neutral Female Accuracy (%)",
        "All Reaction Time (ms)",
        "Angry Male Reaction Time (ms)",
        "Happy Male Reaction Time (ms)",
        "Neutral Male Reaction Time (ms)",
        "Angry Female Reaction Time (ms)",
        "Happy Female Reaction Time (ms)",
        "Neutral Female Reaction Time (ms)",
    )
    .expect("Failed to write to file");
    writeln!(
        output_file,
        "{}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {}, {}, {}, {}, {}, {}, {}",
        input_file_path,
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
    )
    .expect("Failed to write file.");
}
