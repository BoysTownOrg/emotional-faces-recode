#[derive(Debug, PartialEq)]
pub struct Event {
    time_microseconds: i64,
    trigger_code: i32,
}

pub fn parse_events(input: &str) -> Vec<Event> {
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
pub enum Condition {
    Angry,
    Happy,
    Neutral,
}

#[derive(Debug, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, PartialEq)]
pub struct Trial {
    correct_response: bool,
    condition: Condition,
    sex: Sex,
    response_time_milliseconds: Option<i64>,
}

pub fn reconstruct_trials(events: Vec<Event>) -> Vec<Trial> {
    let mut trials = Vec::new();
    let enumerated_nonresponses = events
        .iter()
        .enumerate()
        .filter(|(_, event)| event.trigger_code != 512 && event.trigger_code != 256)
        .collect::<Vec<(usize, &Event)>>();
    let mut response_ready_indices = enumerated_nonresponses
        .windows(2)
        .filter(|window| window[1].1.time_microseconds - window[0].1.time_microseconds > 2500000)
        .map(|window| window[0].0)
        .collect::<Vec<usize>>();
    response_ready_indices.push(enumerated_nonresponses.last().unwrap().0);
    for response_ready_index in response_ready_indices {
        let response = events
            .iter()
            .skip(response_ready_index + 1)
            .find(|event| event.trigger_code == 256 || event.trigger_code == 512);
        let mut correct = false;
        let mut response_time_milliseconds = None;
        let mut condition = Condition::Happy;
        let mut sex = Sex::Male;
        match events[response_ready_index - 1].trigger_code {
            21 => {}
            22 => {
                if let Some(event) = response {
                    correct = event.trigger_code == 512;
                }
                sex = Sex::Female;
                condition = Condition::Happy;
            }
            23 => {
                if let Some(event) = response {
                    correct = event.trigger_code == 512;
                }
                sex = Sex::Female;
                condition = Condition::Neutral;
            }
            31 => {
                if let Some(event) = response {
                    correct = event.trigger_code == 256;
                }
                sex = Sex::Male;
                condition = Condition::Angry;
            }
            32 => {
                if let Some(event) = response {
                    correct = event.trigger_code == 256;
                }
                sex = Sex::Male;
                condition = Condition::Happy;
            }
            33 => {}
            _ => {}
        }
        if let Some(event) = response {
            if correct {
                response_time_milliseconds = Some(
                    (event.time_microseconds - events[response_ready_index].time_microseconds)
                        / 1000,
                );
            }
        }
        trials.push(Trial {
            correct_response: correct,
            condition,
            sex,
            response_time_milliseconds,
        })
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
    fn reconstruct_trials_1() {
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
                response_time_milliseconds: Some(7288 - 6302)
            }],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_2() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 8190000,
                trigger_code: 22,
            },
            Event {
                time_microseconds: 8199000,
                trigger_code: 4118,
            },
            Event {
                time_microseconds: 8888000,
                trigger_code: 512,
            },
            Event {
                time_microseconds: 11342000,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 11352000,
                trigger_code: 4127,
            },
            Event {
                time_microseconds: 11851000,
                trigger_code: 256,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Happy,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(8888 - 8199)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(11851 - 11352)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_3() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 3379000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 5050000,
                trigger_code: 22,
            },
            Event {
                time_microseconds: 5063000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 6402000,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Happy,
                sex: Sex::Female,
                response_time_milliseconds: Some(6402 - 5063)
            }],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_4() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 17681000,
                trigger_code: 23,
            },
            Event {
                time_microseconds: 17691000,
                trigger_code: 4119,
            },
            Event {
                time_microseconds: 18139000,
                trigger_code: 512,
            },
            Event {
                time_microseconds: 20840000,
                trigger_code: 32,
            },
            Event {
                time_microseconds: 20860000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 21298000,
                trigger_code: 256,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Neutral,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(18139 - 17691)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Happy,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(21298 - 20860)
                }
            ],
            trials
        );
    }
}
