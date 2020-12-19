use std::collections::{HashMap, HashSet};

pub mod parse;

pub type RuleIdx = usize;

#[derive(Clone, Debug)]
pub enum Rule {
    And(Vec<RuleIdx>),
    Or(Vec<Vec<RuleIdx>>),
    Terminal(char),
}

impl std::fmt::Display for Rule {
    fn fmt(&self, formater: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rule::And(v) => write!(formater, "{:?}", v),
            Rule::Or(vs) => write!(
                formater,
                "( {} )",
                vs.iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<String>>()
                    .join(" | ")
            ),
            Rule::Terminal(c) => write!(formater, "_{}_", c),
        }
    }
}

impl Rule {
    pub fn matches_to_end(&self, input: &str, rules: &HashMap<RuleIdx, Rule>) -> bool {
        self.matches("0", input, &[0].iter().cloned().collect(), rules, 4)
            .iter()
            .max()
            == Some(&input.len())
    }

    pub fn matches(
        &self,
        my_indx: &str,
        input: &str,
        start_indexes: &HashSet<RuleIdx>,
        rules: &HashMap<RuleIdx, Rule>,
        depth: usize,
    ) -> HashSet<usize> /* new start_idx */ {
        let mut reachable_indexes = HashSet::new();
        if start_indexes.is_empty() {
            return reachable_indexes;
        }
        // println!(
        //     "{:>width$}init [{}]: {} with {:?}",
        //     " ",
        //     my_indx,
        //     self,
        //     start_indexes,
        //     width = depth
        // );

        match self {
            Rule::Terminal(c) => {
                for &start_idx in start_indexes {
                    if let Some(check) = input.chars().nth(start_idx) {
                        if check == *c {
                            reachable_indexes.insert(start_idx + 1);
                        }
                    }
                }
            }
            Rule::And(ref parts) => {
                let mut parts = parts.clone();
                let head = parts.remove(0);
                for start_idx in start_indexes {
                    // find reachable in one step
                    let reachange_in_one_step = Rule::matches(
                        &rules[&head],
                        &head.to_string(),
                        input,
                        &[*start_idx].iter().cloned().collect(),
                        rules,
                        depth + 3,
                    );
                    if parts.is_empty() {
                        reachable_indexes.extend(reachange_in_one_step);
                    } else {
                        reachable_indexes = Rule::matches(
                            &Rule::And(parts.clone()),
                            my_indx,
                            input,
                            &reachange_in_one_step,
                            rules,
                            depth,
                        );
                    }
                }
            }
            Rule::Or(vs) => {
                let mut i = 1;
                for seq in vs {
                    reachable_indexes.extend(&Rule::matches(
                        &Rule::And(seq.clone()),
                        &format!("or{}", i),
                        input,
                        start_indexes,
                        rules,
                        depth + 3,
                    ));
                    i += 1;
                }
            }
        };

        reachable_indexes = reachable_indexes
            .difference(start_indexes)
            .cloned()
            .collect();

        // println!(
        //     "{:>width$}END [{}]: {} with {:?} leads to {:?}",
        //     " ",
        //     my_indx,
        //     self,
        //     start_indexes,
        //     reachable_indexes,
        //     width = depth
        // );

        reachable_indexes
    }
}
