use std::fs;
#[derive(PartialEq)]
enum ReactorType {
    Increment,
    Decrement,
    None,
}

struct Reactor {
    reactor_type: ReactorType,
    data: Vec<i64>,
}

impl Reactor {
    fn new_reactor(input: String) -> Self {
        let reactor = Reactor {
            reactor_type: ReactorType::None,
            data: convert_report(input),
        };
        let typed_reactor = reactor.determine_reactor_type();
        return typed_reactor;
    }

    fn new(input: Vec<i64>) -> Self {
        return Reactor {
            reactor_type: ReactorType::None,
            data: input,
        }.determine_reactor_type()
    }

    fn determine_reactor_type(mut self) -> Self {
        let diff = (self.data[0] - self.data[1]) < 0;
        let reactor_type = match diff {
            true => ReactorType::Increment,
            false => ReactorType::Decrement,
        };
        self.reactor_type = reactor_type;
        return self;
    }

    fn is_reactor_safe(&self) -> bool {
        match self.reactor_type {
            ReactorType::Increment => self
                .data
                .iter()
                .skip(1)
                .fold(Some(self.data[0]), |acc, x| {
                    Reactor::increment_safe(acc, *x)
                })
                .is_some(),
            ReactorType::Decrement => self
                .data
                .iter()
                .skip(1)
                .fold(Some(self.data[0]), |acc, x| {
                    Reactor::decrement_safe(acc, *x)
                })
                .is_some(),
            ReactorType::None => false,
        }
    }

    fn is_reactor_damped(&self) -> bool {
         self.data.iter().enumerate().any(|(x, _)| {
            let mut new_vec = self.data.clone();
            new_vec.remove(x);
            let reactor = Reactor::new(new_vec);
            return reactor.is_reactor_safe();
         })

    }

    fn is_reactor_dampener_safe(&self) -> bool {
        let first = match self.reactor_type {
            ReactorType::Increment => self
                .data
                .iter()
                .skip(1)
                .fold(Some((self.data[0], false)), |acc, x| {
                    Reactor::increment_damp_safe(acc, *x)
                })
                .is_some(),
            ReactorType::Decrement => self
                .data
                .iter()
                .skip(1)
                .fold(Some((self.data[0], false)), |acc, x| {
                    Reactor::decrement_damp_safe(acc, *x)
                })
                .is_some(),
            ReactorType::None => false,
        };
        let diff = self.data[1] - self.data[2] < 0;
        let new_type = match diff {
            true => ReactorType::Increment,
            false => ReactorType::Decrement,
        };
        if new_type != self.reactor_type {
           return match new_type {
                ReactorType::Increment => self.data.iter().skip(2).fold(Some(self.data[1]), | acc,x | Reactor::increment_safe(acc, *x)).is_some(),
                ReactorType::Decrement => self.data.iter().skip(2).fold(Some(self.data[1]), | acc,x | Reactor::decrement_safe(acc, *x)).is_some(),
                ReactorType::None => false
            }
        } else {
            return first;
        }
        
    }

    fn increment_safe(tuple: Option<i64>, next: i64) -> Option<i64> {
        match tuple {
            Some(x) => {
                if (next - x) >= 1 && (next - x) <= 3 {
                    Some(next)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn decrement_safe(tuple: Option<i64>, next: i64) -> Option<i64> {
        match tuple {
            Some(x) => {
                if (x - next) >= 1 && (x - next) <= 3 {
                    Some(next)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn decrement_damp_safe(tuple: Option<(i64, bool)>, next: i64) -> Option<(i64, bool)> {
        println!("Decrement {}, {}, {}", tuple?.0, tuple?.1 , next);
        match tuple {
            Some((x, false)) => {
                if (x - next) >= 1 && (x - next) <= 3 {
                    Some((next, false))
                } else {
                    Some((x, true))
                }
            }
            Some((x, true)) => {
                if (x - next) >= 1 && (x - next) <= 3 {
                    Some((next, true))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn increment_damp_safe(tuple: Option<(i64, bool)>, next: i64) -> Option<(i64, bool)> {
        println!("increment {}, {}, {}", tuple?.0, tuple?.1 , next);
        match tuple {
            Some((x, false)) => {
                if (next - x) >= 1 && (next - x) <= 3 {
                    Some((next, false))
                } else {
                    Some((x, true))
                }
            }
            Some((x, true)) => {
                if (next - x) >= 1 && (next - x) <= 3 {
                    Some((next, true))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("No file found");
    let lines: Vec<String> = content.lines().map(|x| String::from(x)).collect();
    return lines;
}

fn convert_report(report: String) -> Vec<i64> {
    let report: Vec<i64> = report
        .split_whitespace()
        .map(|x| x.parse::<i64>().expect("Something went wrong"))
        .collect();
    return report;
}

fn create_reactors(input: Vec<String>) -> Vec<Reactor> {
    let reactor_farm: Vec<Reactor> = input
        .iter()
        .map(|x| Reactor::new_reactor(x.clone()))
        .collect();
    return reactor_farm;
}

fn main() {
    let input = read_file("input.txt");
    let reactors = create_reactors(input.clone());
    let count = reactors
        .iter()
        .fold(0, |acc, x| if x.is_reactor_safe() { acc + 1 } else { acc });
    let damp_count = create_reactors(input).iter().fold(0, |acc, x| {
        if x.is_reactor_damped() {
            acc + 1
        } else {
            acc
        }
    });
    println!("Safe reports: {} Damped Reports: {}", count, damp_count);
}
