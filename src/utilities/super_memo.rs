use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::entities::Card;

pub fn needs_review(card: &Card) -> bool {
    days_since_seen(card) >= days_until_review(card)
}

fn days_since_seen(card: &Card) -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - card.last_seen / 86400000
}

fn days_until_review(card: &Card) -> u128 {
    sm2(card
        .score
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect())
}

fn sm2(scores: Vec<usize>) -> u128 {
    let a = 6.0;
    let b = -0.8;
    let c = 0.28;
    let d = 0.02;
    let assumed_score = 2.5;
    let min_score = 1.3;
    let theta = 1.0;

    // If not answered, return 0
    if scores.is_empty() {
        return 0;
    }

    // If last incorrect, return 1
    if let Some(score) = scores.last() {
        if *score < 3 {
            return 1;
        }
    }

    let mut streak = 0;
    for i in (0..scores.len()).rev() {
        if scores[i] > 3 {
            streak += 1;
        } else {
            break;
        }
    }

    let history_sum = scores.into_iter().fold(0f64, |acc, cur| {
        acc + (b + (c * cur as f64) + (d * cur as f64 * cur as f64))
    });

    return (a * ((min_score as f64).max(assumed_score + history_sum)).powf(theta * streak as f64))
        as u128;
}
