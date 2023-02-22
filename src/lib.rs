#[derive(Debug, PartialEq)]
struct Event {
    time_microseconds: i64,
    trigger_code: i32,
}

fn parse_events(input: &str) -> Vec<Event> {
    input
        .lines()
        .filter(|&line| line.contains("FIFF Trigger"))
        .map(|line| {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            Event {
                time_microseconds: tokens[0].parse::<i64>().unwrap(),
                trigger_code: tokens[2].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Event>>()
}

#[derive(Debug, PartialEq)]
enum Condition {
    Angry,
    Happy,
    Neutral,
}

#[derive(Debug, PartialEq)]
enum Sex {
    Male,
    Female,
}

#[derive(Debug, PartialEq)]
struct Trial {
    correct_response: bool,
    condition: Condition,
    sex: Sex,
    response_time_milliseconds: Option<i64>,
}

fn reconstruct_trials(events: Vec<Event>) -> Vec<Trial> {
    let mut trials = Vec::new();
    let mut i = 0;
    while i < events.len() - 2 {
        match events[i].trigger_code {
            21 => {}
            22 => {
                let correct = events[i + 2].trigger_code == 512;
                let response_time_milliseconds = if correct {
                    Some(events[i + 2].time_microseconds - events[i + 1].time_microseconds)
                } else {
                    None
                };
                trials.push(Trial {
                    correct_response: correct,
                    condition: Condition::Happy,
                    sex: Sex::Female,
                    response_time_milliseconds,
                });
                i += 1;
            }
            23 => {}
            31 => {}
            32 => {}
            33 => {}
            _ => {}
        }
        i += 1;
    }

    trials
}

#[cfg(test)]
mod tests {
    use crate::{Condition, Event, Sex, Trial};

    #[test]
    fn parse_events() {
        let events = crate::parse_events(
            "Tmu         	Code	TriNo	Comnt	Ver-C
3809479        	11	0	all  6017 0.874 2.19                    
4618000        	1	4096	FIFF Trigger: 4096                      
6293000        	1	22	FIFF Trigger: 22                        
6302000        	1	4118	FIFF Trigger: 4118                      ",
        );
        assert_eq!(
            vec![
                Event {
                    time_microseconds: 4618000,
                    trigger_code: 4096
                },
                Event {
                    time_microseconds: 6293000,
                    trigger_code: 22
                },
                Event {
                    time_microseconds: 6302000,
                    trigger_code: 4118
                }
            ],
            events
        );
    }

    #[test]
    fn reconstruct_trials() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 6293000,
                trigger_code: 22,
            },
            Event {
                time_microseconds: 6302000,
                trigger_code: 4118,
            },
            Event {
                time_microseconds: 7288000,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Happy,
                sex: Sex::Female,
                response_time_milliseconds: Some(7288000 - 6302000)
            }],
            trials
        );
    }
}
