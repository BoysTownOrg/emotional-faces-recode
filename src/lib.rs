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

#[cfg(test)]
mod tests {
    use crate::Event;

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
}
