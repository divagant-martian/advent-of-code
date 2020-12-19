use super::{Rule, RuleIdx};

#[derive(Debug)]
enum State {
    Start,
    IdxParsed,
    VecParsed(Vec<RuleIdx>),
    OrParsed(Vec<Vec<RuleIdx>>),
    TerminalParsed(char),
}

pub fn parse(line: &str) -> (RuleIdx, Rule) {
    let mut state = State::Start;
    let mut idx = None;
    for part in line.split(|c| c == '|' || c == ':').map(|part| part.trim()) {
        match state {
            State::Start => {
                idx = part.parse().ok();
                state = State::IdxParsed;
            }
            State::IdxParsed => {
                if let Ok(vec) = part
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<RuleIdx>())
                    .collect()
                {
                    state = State::VecParsed(vec);
                } else {
                    let c = part.chars().nth(1).unwrap();
                    assert!(c == 'a' || c == 'b', "unexpected terminal");
                    state = State::TerminalParsed(c);
                }
            }
            State::VecParsed(v) => {
                let vec = part
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<RuleIdx>().unwrap())
                    .collect();
                state = State::OrParsed(vec![v, vec]);
                // we got another part, this is an or
            }
            State::OrParsed(mut vs) => {
                let vec = part
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<RuleIdx>().unwrap())
                    .collect();
                vs.push(vec);
                state = State::OrParsed(vs);
            },
            State::TerminalParsed { .. } => {}
        }
    }

    let idx = idx.unwrap();
    match state {
        State::VecParsed(v) => (idx, Rule::And(v)),
        State::OrParsed(vs) => (idx, Rule::Or(vs)),
        State::TerminalParsed(c) => (idx, Rule::Terminal(c)),
        _ => unreachable!("bad rule"),
    }
}
