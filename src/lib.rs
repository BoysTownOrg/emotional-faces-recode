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

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    Angry,
    Happy,
    Neutral,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Trial {
    pub correct_response: bool,
    pub condition: Condition,
    pub sex: Sex,
    pub response_time_milliseconds: Option<i64>,
}

fn has_bit_set(x: i32, n: i32) -> bool {
    let mask = 1 << n;
    (x | mask) == x
}

fn trial_from_response_ready_index(events: &[Event]) -> Trial {
    let response = events
        .iter()
        .skip(2)
        .find(|event| has_bit_set(event.trigger_code, 8) || has_bit_set(event.trigger_code, 9));
    let mut condition = Condition::Happy;
    let mut sex = Sex::Male;
    let mut correct_code = 0;
    let visual_trigger_mask = 1 << 12;
    let combined_triggers = events[0].trigger_code | events[1].trigger_code;
    match combined_triggers & !visual_trigger_mask {
        21 => {
            correct_code = 512;
            sex = Sex::Female;
            condition = Condition::Angry;
        }
        22 => {
            correct_code = 512;
            sex = Sex::Female;
            condition = Condition::Happy;
        }
        23 => {
            correct_code = 512;
            sex = Sex::Female;
            condition = Condition::Neutral;
        }
        31 => {
            correct_code = 256;
            sex = Sex::Male;
            condition = Condition::Angry;
        }
        32 => {
            correct_code = 256;
            sex = Sex::Male;
            condition = Condition::Happy;
        }
        33 => {
            correct_code = 256;
            sex = Sex::Male;
            condition = Condition::Neutral;
        }
        _ => {}
    }
    let mut correct_response = false;
    let mut response_time_milliseconds = None;
    if let Some(event) = response {
        correct_response = event.trigger_code & !visual_trigger_mask == correct_code;
        if correct_response {
            response_time_milliseconds =
                Some((event.time_microseconds - events[1].time_microseconds + 500) / 1000);
        }
    }

    Trial {
        correct_response,
        condition,
        sex,
        response_time_milliseconds,
    }
}

pub fn reconstruct_trials(events: Vec<Event>) -> Vec<Trial> {
    let enumerated_nonresponses = events
        .iter()
        .enumerate()
        .filter(|(_, event)| {
            let button1_mask = 1 << 8;
            let button2_mask = 1 << 9;
            event.trigger_code & (button1_mask | button2_mask) == 0
        })
        .collect::<Vec<_>>();
    let start_of_trials_indices = enumerated_nonresponses
        .windows(2)
        .filter(|window| {
            let first_event = window[0].1;
            let second_event = window[1].1;
            let difference_time_microseconds =
                second_event.time_microseconds - first_event.time_microseconds;
            (difference_time_microseconds < 100_000
                && (has_bit_set(first_event.trigger_code, 12)
                    || has_bit_set(second_event.trigger_code, 12)))
                || difference_time_microseconds > 10_000_000
        })
        .map(|window| {
            let first_event_index = window[0].0;
            first_event_index
        })
        .collect::<Vec<_>>();
    let mut trials = start_of_trials_indices
        .windows(2)
        .map(|indices| trial_from_response_ready_index(&events[indices[0]..indices[1]]))
        .collect::<Vec<_>>();
    trials.push(trial_from_response_ready_index(
        &events[*start_of_trials_indices.last().unwrap()..],
    ));
    trials
}

pub fn accuracy_percentage(trials: &[Trial]) -> f64 {
    100. * trials.iter().filter(|trial| trial.correct_response).count() as f64 / trials.len() as f64
}

pub fn reaction_time_milliseconds(trials: &[Trial]) -> Option<i64> {
    let correct_trials = trials
        .iter()
        .filter(|trial| trial.correct_response)
        .collect::<Vec<_>>();
    let count = correct_trials.len() as i64;
    if count == 0 {
        None
    } else {
        Some(
            (correct_trials
                .iter()
                .map(|trial| trial.response_time_milliseconds.unwrap())
                .sum::<i64>()
                + count / 2)
                / count,
        )
    }
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
    fn reconstruct_trials_preliminary_4096() {
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
    fn reconstruct_trials_preliminary_4096_earlier() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 2543000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 6207000,
                trigger_code: 22,
            },
            Event {
                time_microseconds: 6211000,
                trigger_code: 4118,
            },
            Event {
                time_microseconds: 7104000,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Happy,
                sex: Sex::Female,
                response_time_milliseconds: Some(7104 - 6211)
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

    #[test]
    fn reconstruct_trials_5() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 27193000,
                trigger_code: 21,
            },
            Event {
                time_microseconds: 27207000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 27724000,
                trigger_code: 512,
            },
            Event {
                time_microseconds: 30242000,
                trigger_code: 33,
            },
            Event {
                time_microseconds: 30259000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 30762000,
                trigger_code: 256,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(27724 - 27207)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Neutral,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(30762 - 30259)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_6() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 124552000,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 124555000,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 125153000,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Angry,
                sex: Sex::Female,
                response_time_milliseconds: Some(125153 - 124555)
            }],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_7() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 374785984,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 374798016,
                trigger_code: 4127,
            },
            Event {
                time_microseconds: 375347008,
                trigger_code: 256,
            },
            Event {
                time_microseconds: 377984000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 393036000,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 393732992,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(549)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(697)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_response_of_7936() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 299367008,
                trigger_code: 23,
            },
            Event {
                time_microseconds: 299380992,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 299999008,
                trigger_code: 7936,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: false,
                condition: Condition::Neutral,
                sex: Sex::Female,
                response_time_milliseconds: None
            },],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_double_response() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 558014976,
                trigger_code: 23,
            },
            Event {
                time_microseconds: 558033024,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 558448000,
                trigger_code: 256,
            },
            Event {
                time_microseconds: 558939008,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: false,
                condition: Condition::Neutral,
                sex: Sex::Female,
                response_time_milliseconds: None
            },],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_missing_response() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 636076032,
                trigger_code: 22,
            },
            Event {
                time_microseconds: 636081984,
                trigger_code: 4118,
            },
            Event {
                time_microseconds: 639188992,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 639201024,
                trigger_code: 4127,
            },
            Event {
                time_microseconds: 639708032,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: false,
                    condition: Condition::Happy,
                    sex: Sex::Female,
                    response_time_milliseconds: None
                },
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: None
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_duplicate_trigger() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 696003968,
                trigger_code: 21,
            },
            Event {
                time_microseconds: 696014976,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 696262016,
                trigger_code: 512,
            },
            Event {
                time_microseconds: 699091008,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 699091968,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 699100992,
                trigger_code: 4127,
            },
            Event {
                time_microseconds: 699731008,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(247)
                },
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: None
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_trial_begins_with_visual() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 379575008,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 394620000,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 395347008,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Angry,
                sex: Sex::Female,
                response_time_milliseconds: Some(727)
            }],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_triple_response() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 730987008,
                trigger_code: 23,
            },
            Event {
                time_microseconds: 730995008,
                trigger_code: 4119,
            },
            Event {
                time_microseconds: 731918016,
                trigger_code: 256,
            },
            Event {
                time_microseconds: 732334016,
                trigger_code: 512,
            },
            Event {
                time_microseconds: 732572032,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: false,
                condition: Condition::Neutral,
                sex: Sex::Female,
                response_time_milliseconds: None
            }],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_duplicate_trigger_again() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 682076992,
                trigger_code: 21,
            },
            Event {
                time_microseconds: 682089984,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 682934016,
                trigger_code: 512,
            },
            Event {
                time_microseconds: 685257984,
                trigger_code: 33,
            },
            Event {
                time_microseconds: 685259008,
                trigger_code: 33,
            },
            Event {
                time_microseconds: 685276032,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 686092032,
                trigger_code: 256,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(844)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Neutral,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(686092 - 685276)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_response_of_768() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 122190000,
                trigger_code: 21,
            },
            Event {
                time_microseconds: 122201000,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 122553000,
                trigger_code: 256,
            },
            Event {
                time_microseconds: 122841000,
                trigger_code: 768,
            },
            Event {
                time_microseconds: 125278000,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 125287000,
                trigger_code: 4127,
            },
            Event {
                time_microseconds: 125783000,
                trigger_code: 256,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: None
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(125783 - 125287)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_responses_masked_by_visual() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 75647000,
                trigger_code: 23,
            },
            Event {
                time_microseconds: 75655000,
                trigger_code: 4119,
            },
            Event {
                time_microseconds: 76278000,
                trigger_code: 4608,
            },
            Event {
                time_microseconds: 78691000,
                trigger_code: 32,
            },
            Event {
                time_microseconds: 78706000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 79444000,
                trigger_code: 4352,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Neutral,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(76278 - 75655)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Happy,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(79444 - 78706)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_lone_visual() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 185392000,
                trigger_code: 33,
            },
            Event {
                time_microseconds: 185410000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 186028000,
                trigger_code: 256,
            },
            Event {
                time_microseconds: 187780000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 188596000,
                trigger_code: 33,
            },
            Event {
                time_microseconds: 188612000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 189060000,
                trigger_code: 4352,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Neutral,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(186028 - 185410)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Neutral,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(189060 - 188612)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_extra_visual() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 548185984,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 548195008,
                trigger_code: 4127,
            },
            Event {
                time_microseconds: 548929984,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 549126976,
                trigger_code: 4352,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Angry,
                sex: Sex::Male,
                response_time_milliseconds: Some(932)
            }],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_lone_visual_2() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 626684032,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 626700992,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 627206016,
                trigger_code: 256,
            },
            Event {
                time_microseconds: 628779008,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 629913024,
                trigger_code: 22,
            },
            Event {
                time_microseconds: 629936000,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 630563008,
                trigger_code: 4608,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(505)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Happy,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(627)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_lone_visual_3() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 689667008,
                trigger_code: 23,
            },
            Event {
                time_microseconds: 689683008,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 690217024,
                trigger_code: 512,
            },
            Event {
                time_microseconds: 690545984,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 692801024,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 692817984,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 693379008,
                trigger_code: 4352,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Neutral,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(534)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(561)
                }
            ],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_4097() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 663971968,
                trigger_code: 33,
            },
            Event {
                time_microseconds: 663984000,
                trigger_code: 4097,
            },
            Event {
                time_microseconds: 664692992,
                trigger_code: 256,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Neutral,
                sex: Sex::Male,
                response_time_milliseconds: Some(709)
            },],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_close_triggers() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 720435968,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 720436992,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 721166976,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![Trial {
                correct_response: true,
                condition: Condition::Angry,
                sex: Sex::Female,
                response_time_milliseconds: Some(730)
            },],
            trials
        );
    }

    #[test]
    fn reconstruct_trials_15_second_break() {
        let trials = crate::reconstruct_trials(vec![
            Event {
                time_microseconds: 376816000,
                trigger_code: 31,
            },
            Event {
                time_microseconds: 376830016,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 377276992,
                trigger_code: 256,
            },
            Event {
                time_microseconds: 379982016,
                trigger_code: 4096,
            },
            Event {
                time_microseconds: 395027008,
                trigger_code: 4117,
            },
            Event {
                time_microseconds: 395624000,
                trigger_code: 512,
            },
        ]);
        assert_eq!(
            vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(447)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(597)
                },
            ],
            trials
        );
    }

    #[test]
    fn accuracy() {
        assert_eq!(
            75.,
            crate::accuracy_percentage(&vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(447)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(597)
                },
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(447)
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(597)
                },
            ])
        )
    }

    #[test]
    fn reaction_time() {
        assert_eq!(
            Some((447 + 214 + 1) / 2),
            crate::reaction_time_milliseconds(&vec![
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: Some(447)
                },
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: None
                },
                Trial {
                    correct_response: true,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: Some(214)
                },
            ])
        )
    }

    #[test]
    fn reaction_time_all_wrong() {
        assert_eq!(
            None,
            crate::reaction_time_milliseconds(&vec![
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: None
                },
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Male,
                    response_time_milliseconds: None
                },
                Trial {
                    correct_response: false,
                    condition: Condition::Angry,
                    sex: Sex::Female,
                    response_time_milliseconds: None
                },
            ])
        )
    }
}
