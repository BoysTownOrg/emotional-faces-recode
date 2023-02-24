use emotional_faces_recode::{
    accuracy_percentage, parse_events, reconstruct_trials, Condition, Sex, Trial,
};
use std::io::Write;

fn trials_matching(trials: &[Trial], condition: Condition, sex: Sex) -> Vec<Trial> {
    trials
        .iter()
        .cloned()
        .filter(|trial| trial.condition == condition && trial.sex == sex)
        .collect::<Vec<_>>()
}

fn reaction_time_milliseconds(trials: &[Trial]) -> String {
    if let Some(t) = emotional_faces_recode::reaction_time_milliseconds(trials) {
        t.to_string()
    } else {
        "NaN".to_string()
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_directory = &args[1];
    let output_file_path = &args[2];
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
    for file in std::fs::read_dir(input_directory).unwrap() {
        let path = file.unwrap().path();
        let extension = path.extension().unwrap_or(std::ffi::OsStr::new(""));
        if extension != "evt" {
            continue;
        }
        let contents = std::fs::read_to_string(&path).unwrap();
        let events = parse_events(&contents);
        let trials = reconstruct_trials(events);
        if trials.len() != 240 {
            println!("Unexpected number of trials: {}", trials.len());
            println!("Skipping.");
        }
        writeln!(
            output_file,
            "{}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {}, {}, {}, {}, {}, {}, {}",
            path.to_str().unwrap(),
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
}
