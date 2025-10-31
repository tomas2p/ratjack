use crate::game::deck::crear_baraja;
use crate::game::logic::jugar_turno;
use crate::game::player::Jugador;
use crate::strategies::{should_draw, strategy_label, Strategy};

#[derive(Debug)]
pub struct Summary {
    pub reps: u32,
    pub num_players: usize,
    pub wins: Vec<u32>,
    pub wins_per_game: Vec<u32>,
    pub per_game_ties: u32,
    pub busts: Vec<u32>,
    pub total_points: Vec<u64>,
    pub ties: u32,
    pub strat_labels: Vec<String>,
}

// Simulate `reps` games with `num_players` using provided strategies vector.
// This function is crate-visible (pub(crate)) so it can be called from the UI, but we avoid exposing
// it as a public CLI helper to encourage running simulations through the UI.
pub(crate) fn simulate(reps: u32, num_players: usize, strategies: Vec<Strategy>) -> Summary {
    let num_players = num_players.clamp(2, 8);

    // Build strategy vector
    let mut strat_vec: Vec<Strategy> = Vec::with_capacity(num_players);
    let mut strat_labels: Vec<String> = Vec::with_capacity(num_players);
    for i in 0..num_players {
        if let Some(s) = strategies.get(i) {
            strat_vec.push(s.clone());
        } else if !strategies.is_empty() {
            strat_vec.push(strategies.last().unwrap().clone());
        } else {
            let t = 12 + ((i % 8) as u8);
            strat_vec.push(Strategy::Threshold(t));
        }
        strat_labels.push(strategy_label(&strat_vec[i]));
    }

    // Only per-partida metrics: wins_per_game counts sole victories; per_game_ties counts games with multiple winners
    let mut wins = vec![0u32; num_players];
    let mut wins_per_game = vec![0u32; num_players];
    let mut per_game_ties = 0u32;
    let mut busts = vec![0u32; num_players];
    let mut total_points = vec![0u64; num_players];
    // `ties` (per-enfrentamiento) is deprecated in this mode; keep for compatibility but will remain 0
    let mut ties = 0u32;

    for _ in 0..reps {
        let mut baraja = crear_baraja();

        // Create players
        let mut jugadores: Vec<Jugador> = (0..num_players)
            .map(|i| {
                let mut j = Jugador::nuevo();
                j.nombre = format!("P{}", i + 1);
                j
            })
            .collect();

        // initial deal
        for _ in 0..2 {
            for p in jugadores.iter_mut() {
                p.tomar_carta(&mut baraja);
            }
        }
        for p in jugadores.iter_mut() {
            p.puntos = p.puntaje();
        }

        // players act
        for idx in 0..num_players {
            loop {
                let pts = jugadores[idx].puntaje();
                if should_draw(&strat_vec[idx], pts, &baraja) {
                    jugar_turno(&mut jugadores[idx], &mut baraja, true);
                    jugadores[idx].puntos = jugadores[idx].puntaje();
                    if jugadores[idx].puntaje() > 21 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        // evaluate: all-vs-all pairwise comparisons
        // total_points and bust counters per player
        for i in 0..num_players {
            let pts = jugadores[i].puntaje();
            total_points[i] += pts as u64;
            if pts > 21 {
                busts[i] += 1;
            }
        }

        // per-game outcome only (no enfrentamientos): determine top scorers ≤21
        let mut best_pts: i32 = -1;
        for i in 0..num_players {
            let pts = jugadores[i].puntaje() as i32;
            if pts <= 21 && pts > best_pts {
                best_pts = pts;
            }
        }
        if best_pts >= 0 {
            // collect winners (could be multiple -> tie)
            let mut winners: Vec<usize> = Vec::new();
            for i in 0..num_players {
                if jugadores[i].puntaje() as i32 == best_pts {
                    winners.push(i);
                }
            }
            if winners.len() == 1 {
                wins_per_game[winners[0]] += 1;
                wins[winners[0]] += 1; // keep wins aligned with per-game wins
            } else {
                // multiple winners -> count as per-game tie
                per_game_ties += 1;
            }
        } else {
            // all busted -> count as tied game
            per_game_ties += 1;
        }
    }

    Summary {
        reps,
        num_players,
        wins,
        wins_per_game,
        per_game_ties,
        busts,
        total_points,
        ties,
        strat_labels,
    }
}

pub fn print_summary(summary: &Summary) {
    for line in summary_lines(summary) {
        println!("{}", line);
    }
}

pub fn summary_lines(summary: &Summary) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Resumen final después de {} partidas:",
        summary.reps
    ));
    lines.push(String::new());

    // Table header: per-partida victories and percentage
    lines.push(format!(
        "{:<3} {:<30} {:>10} {:>8} {:>10}",
        "ID", "Estrategia", "Vict(part)", "%P", "PtsAvg"
    ));
    lines.push("-".repeat(70));

    for i in 0..summary.num_players {
        let id = format!("{}", i + 1);
        let strat = summary.strat_labels.get(i).cloned().unwrap_or_default();
        let vict_part = summary.wins_per_game[i];
        let pct = if summary.reps > 0 {
            (vict_part as f64) / (summary.reps as f64) * 100.0
        } else {
            0.0
        };
        let pts_avg = if summary.reps > 0 {
            (summary.total_points[i] as f64) / (summary.reps as f64)
        } else {
            0.0
        };
        lines.push(format!(
            "{:<3} {:<30} {:>10} {:>7.2} {:>10.2}",
            id, strat, vict_part, pct, pts_avg
        ));
    }

    lines.push(String::new());
    lines.push(format!(
        "Partidas empatadas (múltiples ganadores): {}",
        summary.per_game_ties
    ));

    // Determine best strategies by metric
    let mut best_by_partida = 0usize;
    let mut best_by_partida_pct = -1.0f64;
    let mut best_by_enf = 0usize;
    let mut best_by_enf_pct = -1.0f64;
    for i in 0..summary.num_players {
        let pct_partida = if summary.reps > 0 {
            (summary.wins_per_game[i] as f64) / (summary.reps as f64) * 100.0
        } else {
            0.0
        };
        if pct_partida > best_by_partida_pct {
            best_by_partida_pct = pct_partida;
            best_by_partida = i;
        }
        // per-opponent percentage: each player has (num_players-1) opponents per game
        let denom = (summary.reps as f64) * ((summary.num_players - 1) as f64);
        let pct_enf = if denom > 0.0 {
            (summary.wins[i] as f64) / denom * 100.0
        } else {
            0.0
        };
        if pct_enf > best_by_enf_pct {
            best_by_enf_pct = pct_enf;
            best_by_enf = i;
        }
    }

    let label_partida = &summary.strat_labels[best_by_partida];
    let label_enf = &summary.strat_labels[best_by_enf];
    lines.push(String::new());
    lines.push(format!(
        "Mejor por partidas: {} ({:.2}%)",
        label_partida, best_by_partida_pct
    ));
    lines.push(format!(
        "Mejor por enfrentamientos: {} ({:.2}%)",
        label_enf, best_by_enf_pct
    ));
    if best_by_partida == best_by_enf {
        lines.push(format!(
            "Conclusión: estrategia consistente — '{}' domina ambas métricas.",
            label_partida
        ));
    } else {
        lines.push(format!("Conclusión:"));
        lines.push(format!(
            " - Si te interesa ganar partidas completas (mayoría en cada partida) prioriza '{}' ({:.2}%).",
            label_partida, best_by_partida_pct
        ));
        lines.push(format!(
            " - Si prefieres maximizar victorias por enfrentamiento (cada mano contra la banca) prioriza '{}' ({:.2}%).",
            label_enf, best_by_enf_pct
        ));
    }

    lines
}
