#[derive(Copy, Debug, Clone, PartialEq)]
enum Title {
    Respondent,
    Manager,
    Director,
}

#[derive(Copy, Debug, Clone, PartialEq)]
enum State {
    Available,
    Unavailable,
}

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    title: Title,
    state: State,
}

#[derive(Debug, Clone)]
struct Call {
    from: String,
}

struct CallCentre {
    employees: Vec<Employee>,
}

impl CallCentre {
    fn find_next_available(&self, title: Title) -> Option<&Employee> {
        self.employees
            .iter()
            .find(|&employee| employee.title == title && employee.state == State::Available)
    }

    fn dispatch_call(&self, _call: Call) -> Option<&Employee> {
        if let Some(respondent) = self.find_next_available(Title::Respondent) {
            Some(respondent)
        } else if let Some(manager) = self.find_next_available(Title::Manager) {
            Some(manager)
        } else if let Some(director) = self.find_next_available(Title::Director) {
            Some(director)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_centre() {
        let call_centre = CallCentre {
            employees: vec![
                Employee {
                    name: "Bob".to_string(),
                    title: Title::Respondent,
                    state: State::Unavailable,
                },
                Employee {
                    name: "Jane".to_string(),
                    title: Title::Manager,
                    state: State::Available,
                },
            ],
        };
        let employee = call_centre
            .dispatch_call(Call {
                from: "Obama".to_string(),
            })
            .unwrap();
        assert_eq!(employee.name, "Jane");
    }
}

fn main() {
    let call_centre = CallCentre {
        employees: vec![
            Employee {
                name: "Bob".to_string(),
                title: Title::Respondent,
                state: State::Unavailable,
            },
            Employee {
                name: "Jane".to_string(),
                title: Title::Manager,
                state: State::Available,
            },
        ],
    };
    let _employee = call_centre
        .dispatch_call(Call {
            from: "Obama".to_string(),
        })
        .unwrap();
}
