use crate::liquids::LiquidColors;
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone)]
pub struct Cup {
    pub capacity: usize,
    pub liquids: Vec<LiquidColors>,
}

impl fmt::Display for Cup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let liquids_str = self
            .liquids
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "Cup ({}), {}", self.capacity, liquids_str)
    }
}

pub fn pour_a_into_b(a: &Cup, b: &Cup) -> Result<(Cup, Cup), String> {
    if std::ptr::eq(a, b) {
        return Err("Cannot pour a cup into itself".to_string());
    }
    if b.capacity == b.liquids.len() {
        return Err("Destination cup has no space x_x".to_string());
    }
    if a.liquids.is_empty() {
        return Err("Source cup has no liquid to give".to_string());
    }
    let a_top = a.liquids.last().unwrap();
    let b_top = b.liquids.last();
    if b_top.is_some() && b_top.unwrap() != a_top {
        return Err("Source and destination colors do not match".to_string());
    }
    let mut new_a_liquids = a.liquids.clone();
    let mut new_b_liquids = b.liquids.clone();
    let moving_liquid = new_a_liquids.pop();
    new_b_liquids.push(moving_liquid.unwrap());
    let cup_one = Cup {
        capacity: a.capacity,
        liquids: new_a_liquids,
    };
    let cup_two = Cup {
        capacity: b.capacity,
        liquids: new_b_liquids,
    };
    Ok((cup_one, cup_two))
}

fn unrestricted_pour_a_into_b(a: &Cup, b: &Cup) -> Result<(Cup, Cup), String> {
    if std::ptr::eq(a, b) {
        return Err("Cannot pour a cup into itself".to_string());
    }
    if b.capacity <= b.liquids.len() {
        return Err("Destination cup has no space x_x".to_string());
    }
    if a.liquids.is_empty() {
        return Err("Source cup has no liquid to give".to_string());
    }
    let mut new_a = a.clone();
    let mut new_b = b.clone();
    if let Some(moving_liquid) = new_a.liquids.pop() {
        new_b.liquids.push(moving_liquid);
    }
    Ok((new_a, new_b))
}

pub fn scramble_cups() -> Vec<Cup> {
    let mut cups = vec![
        Cup {
            capacity: 5,
            liquids: vec![
                LiquidColors::Red,
                LiquidColors::Red,
                LiquidColors::Red,
                LiquidColors::Red,
                LiquidColors::Red,
            ],
        },
        Cup {
            capacity: 5,
            liquids: vec![
                LiquidColors::Green,
                LiquidColors::Green,
                LiquidColors::Green,
                LiquidColors::Green,
                LiquidColors::Green,
            ],
        },
        Cup {
            capacity: 5,
            liquids: vec![
                LiquidColors::Blue,
                LiquidColors::Blue,
                LiquidColors::Blue,
                LiquidColors::Blue,
                LiquidColors::Blue,
            ],
        },
        Cup {
            capacity: 5,
            liquids: vec![],
        },
        Cup {
            capacity: 5,
            liquids: vec![],
        },
    ];
    let iterations = 10000;
    for _ in 0..iterations {
        let src_idx = rand::random_range(0..cups.len());
        let dst_idx = rand::random_range(0..cups.len());
        if let Ok((new_src, new_dst)) = unrestricted_pour_a_into_b(&cups[src_idx], &cups[dst_idx]) {
            cups[src_idx] = new_src;
            cups[dst_idx] = new_dst;
        }
    }
    cups
}

pub fn are_cups_solved(cups: &[Cup]) -> bool {
    let mut seen_colors: HashSet<LiquidColors> = HashSet::new();
    for c in cups.iter() {
        // First check that the color has not already been seen in another cup
        // if it has not then add it to the set
        if let Some(l) = c.liquids.last() {
            if seen_colors.contains(l) {
                return false;
            }
            seen_colors.insert(l.clone());
        }

        // Next ensure all colors in the cup are the same
        let mut last_liquid: Option<LiquidColors> = None;
        for l in c.liquids.iter() {
            if let Some(l_liq) = last_liquid {
                if &l_liq != l {
                    return false;
                }
            }
            last_liquid = Some(l.clone());
        }
    }
    true
}

#[cfg(test)]
mod test_pour_a_into_b {
    use super::*;

    #[test]
    fn pour_into_self() {
        let c = Cup {
            capacity: 5,
            liquids: vec![],
        };
        let r = pour_a_into_b(&c, &c);
        match r {
            Ok(_) => panic!("Expected an err but got Ok"),
            Err(msg) => assert_eq!(msg, "Cannot pour a cup into itself"),
        }
    }

    #[test]
    fn two_empty_cups() {
        let empty_cup = Cup {
            capacity: 5,
            liquids: Vec::new(),
        };
        let r = pour_a_into_b(&empty_cup.clone(), &empty_cup.clone());
        match r {
            Ok(_) => panic!("Expected an err but got Ok"),
            Err(msg) => assert_eq!(msg, "Source cup has no liquid to give"),
        }
    }

    #[test]
    fn non_matching_cups() {
        let r = pour_a_into_b(
            &Cup {
                capacity: 5,
                liquids: vec![LiquidColors::Red],
            },
            &Cup {
                capacity: 5,
                liquids: vec![LiquidColors::Green],
            },
        );
        match r {
            Ok(_) => panic!("Expected an err but got Ok"),
            Err(msg) => assert_eq!(msg, "Source and destination colors do not match"),
        }
    }

    #[test]
    fn no_space() {
        let r = pour_a_into_b(
            &Cup {
                capacity: 1,
                liquids: vec![LiquidColors::Red],
            },
            &Cup {
                capacity: 1,
                liquids: vec![LiquidColors::Green],
            },
        );
        match r {
            Ok(_) => panic!("Expected an err but got Ok"),
            Err(msg) => assert_eq!(msg, "Destination cup has no space x_x"),
        }
    }

    #[test]
    fn good_pour() {
        let r = pour_a_into_b(
            &Cup {
                capacity: 5,
                liquids: vec![LiquidColors::Red, LiquidColors::Green, LiquidColors::Blue],
            },
            &Cup {
                capacity: 5,
                liquids: vec![LiquidColors::Green, LiquidColors::Green, LiquidColors::Blue],
            },
        );
        match r {
            Ok((c_one, c_two)) => {
                assert_eq!(c_one.liquids, vec![LiquidColors::Red, LiquidColors::Green]);
                assert_eq!(
                    c_two.liquids,
                    vec![
                        LiquidColors::Green,
                        LiquidColors::Green,
                        LiquidColors::Blue,
                        LiquidColors::Blue
                    ]
                );
            }
            Err(_) => panic!("Expected r to be Ok(..)"),
        }
    }
}
