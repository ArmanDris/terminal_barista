use crate::cup::{Cup, scramble_cups};
use crate::liquids::LiquidColors;

const CAPACITY: usize = 5;

pub enum SimpleDifficulties {
    Easy,
    Medium,
    Hard,
}

pub fn get_full_cup(color: LiquidColors) -> Cup {
    let mut liquids = vec![];
    for i in 0..CAPACITY {
        liquids.push(color.clone());
    }
    return Cup {
        capacity: CAPACITY,
        liquids: liquids,
    }
}

fn generate_easy_cups() -> Vec<Cup> {
    let mut cups = vec![
        get_full_cup(LiquidColors::Red),
        get_full_cup(LiquidColors::Green)
    ];
    scramble_cups(cups.clone());
    cups.push(Cup {
        capacity: CAPACITY,
        liquids: vec![],
    });

    return cups;
}

fn generate_medium_cups() -> Vec<Cup> {
    let mut cups = vec![
        get_full_cup(LiquidColors::Red),
        get_full_cup(LiquidColors::Green),
        get_full_cup(LiquidColors::Blue),
        get_full_cup(LiquidColors::BabyBlue),
        get_full_cup(LiquidColors::Pink),
        get_full_cup(LiquidColors::Yellow),
    ];
    scramble_cups(cups.clone());
    cups.push(Cup {
        capacity: CAPACITY,
        liquids: vec![],
    });

    cups.push(Cup {
        capacity: CAPACITY,
        liquids: vec![],
    });

    return cups;
}

fn generate_hard_cups() -> Vec<Cup> {
    let mut cups = vec![
        get_full_cup(LiquidColors::Red),
        get_full_cup(LiquidColors::Green),
        get_full_cup(LiquidColors::Blue),
        get_full_cup(LiquidColors::BabyBlue),
        get_full_cup(LiquidColors::Pink),
        get_full_cup(LiquidColors::Yellow),
        get_full_cup(LiquidColors::Green),
        get_full_cup(LiquidColors::Pink),
    ];

    scramble_cups(cups.clone());
    cups.push(Cup {
        capacity: CAPACITY,
        liquids: vec![],
    });

    cups.push(Cup {
        capacity: CAPACITY,
        liquids: vec![],
    });

    return cups;

}

pub fn generate_cups(diff: SimpleDifficulties) -> Vec<Cup> {
    // Easy: 3 cups, 1 empty, 2 colors
    // Medium: 6 cups, 2 empty, 4 colors
    // Hard: 8 cups, 2 empty, 6 colors
    return match diff {
        SimpleDifficulties::Easy => generate_easy_cups(),
        SimpleDifficulties::Medium => generate_medium_cups(),
        SimpleDifficulties::Hard => generate_hard_cups()
    }
}
