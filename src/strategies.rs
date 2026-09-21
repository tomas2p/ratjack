use crate::game::deck::Carta;
use rand::prelude::*;
use rand::seq::index;

#[derive(Clone, Debug)]
pub enum Strategy {
    Threshold(u8),      // plantarse en >= N
    Random(f64),        // sacar con probabilidad p
    Prob(f64),          // sacar si probabilidad de bust < p
    Count { base: u8 }, // Hi-Lo count based decision
}

pub fn parse_strategies(s: &str) -> Vec<Strategy> {
    if s.trim().is_empty() {
        return Vec::new();
    }
    s.split(',')
        .map(|tok| {
            let tok = tok.trim();
            if tok.starts_with("threshold:") {
                tok[10..]
                    .parse::<u8>()
                    .ok()
                    .map(Strategy::Threshold)
                    .unwrap_or(Strategy::Threshold(17))
            } else if tok.starts_with("random:") {
                tok[7..]
                    .parse::<f64>()
                    .ok()
                    .map(Strategy::Random)
                    .unwrap_or(Strategy::Random(0.5))
            } else if tok.starts_with("prob:") {
                tok[5..]
                    .parse::<f64>()
                    .ok()
                    .map(Strategy::Prob)
                    .unwrap_or(Strategy::Prob(0.5))
            } else if tok.starts_with("count:") {
                tok[6..]
                    .parse::<u8>()
                    .ok()
                    .map(|b| Strategy::Count { base: b })
                    .unwrap_or(Strategy::Count { base: 17 })
            } else if let Ok(n) = tok.parse::<u8>() {
                Strategy::Threshold(n)
            } else {
                Strategy::Threshold(17)
            }
        })
        .collect()
}

// Calcula la probabilidad de bust al simular draws hasta un límite.
pub fn calc_bust_probability_simulation(
    player_points: u8,
    deck: &[Carta],
    stand_at: u8,
    sims: usize,
    max_draws: usize,
) -> f64 {
    if deck.is_empty() {
        return 1.0;
    }
    let mut rng = thread_rng();
    let mut busts = 0usize;

    // Pre-extract values to avoid matching/method calls in inner loop
    let deck_values: Vec<u8> = deck.iter().map(|c| c.valor).collect();
    let n = deck_values.len();
    let draws_to_sim = max_draws.min(n);

    for _ in 0..sims {
        let mut points = player_points as i32;
        let indices = index::sample(&mut rng, n, draws_to_sim);

        for idx in indices.iter() {
            if points >= stand_at as i32 {
                break;
            }

            let val = val_to_points(deck_values[idx], points as u8) as i32;
            points += val;

            if points > 21 {
                break;
            }
        }

        if points > 21 {
            busts += 1;
        }
    }

    busts as f64 / sims as f64
}

// Helper optimizado para obtener puntos de un valor de carta (1-13)
fn val_to_points(valor: u8, current_points: u8) -> u8 {
    if valor == 1 {
        if current_points <= 10 {
            11
        } else {
            1
        }
    } else if valor >= 11 {
        10
    } else {
        valor
    }
}

// Hi-Lo value para una carta (2-6 +1, 7-9 0, 10-A -1)
fn hi_lo_value(valor: u8) -> i32 {
    match valor {
        2..=6 => 1,
        7..=9 => 0,
        10..=13 | 1 => -1,
        _ => 0,
    }
}

pub fn calc_true_count_from_remaining(deck: &[Carta]) -> f64 {
    if deck.is_empty() {
        return 0.0;
    }
    let mut running_rem: i32 = 0;
    for c in deck {
        running_rem += hi_lo_value(c.valor);
    }
    let running_seen = -(running_rem as f64);
    let decks_remaining = (deck.len() as f64) / 52.0;
    if decks_remaining <= 0.0 {
        0.0
    } else {
        running_seen / decks_remaining
    }
}

pub fn should_draw(strategy: &Strategy, player_points: u8, deck: &[Carta]) -> bool {
    match strategy {
        Strategy::Threshold(n) => player_points < *n,
        Strategy::Random(p) => thread_rng().gen::<f64>() < *p,
        Strategy::Prob(p) => {
            let prob = calc_bust_probability_simulation(player_points, deck, 17, 400, 4);
            prob < *p
        }
        Strategy::Count { base } => {
            let tc = calc_true_count_from_remaining(deck);
            let eff = (*base as i32) - tc.round() as i32;
            player_points < (eff.clamp(12, 21) as u8)
        }
    }
}

pub fn strategy_label(strategy: &Strategy) -> String {
    match strategy {
        Strategy::Threshold(n) => format!("Threshold({})", n),
        Strategy::Random(p) => format!("Random({:.2})", p),
        Strategy::Prob(p) => format!("Prob({:.2})", p),
        Strategy::Count { base } => format!("Count({})", base),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::{Carta, Palo};

    #[test]
    fn test_parse_strategies() {
        let s = "threshold:16,random:0.2,prob:0.3,count:18";
        let v = parse_strategies(s);
        assert_eq!(v.len(), 4);
        if let Strategy::Threshold(n) = v[0] {
            assert_eq!(n, 16);
        } else {
            panic!("Expected Threshold");
        }
    }

    #[test]
    fn test_calc_bust_prob() {
        let deck = vec![
            Carta {
                valor: 10,
                palo: Palo::Corazones
            };
            20
        ];
        // si tengo 15 puntos y pido una de 10, busteo seguro.
        let p = calc_bust_probability_simulation(15, &deck, 21, 100, 1);
        assert!(p > 0.9);
    }
}
