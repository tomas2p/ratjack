use std::env;

#[derive(Debug, Clone)]
pub enum Mode {
    Ui,
    AutoUi,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub mode: Mode,
    pub reps: u32,
    pub num_players: usize,
    pub strategies_raw: String,
    pub ui_str_raw: String,
    pub show_help: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Ui,
            reps: 1000,
            num_players: 2,
            strategies_raw: String::new(),
            ui_str_raw: String::new(),
            show_help: false,
        }
    }
}

// Very small and forgiving CLI parser. Supported forms:
// --auto <reps> <players> [--strategies "s1,s2"]
// --auto-ui <reps> <players> [--strategies "s1,s2"]
// default UI: no flags or --ui with optional --ui-strategies
pub fn parse_from(args_in: Vec<String>) -> Config {
    let mut cfg = Config::default();
    let mut args = args_in;

    // Normalize combined forms like --strategies=... and --ui-strategies=
    let mut i = 0;
    while i < args.len() {
        if args[i].starts_with("--strategies=") {
            let v = args[i][13..].to_string();
            args[i] = "--strategies".to_string();
            args.insert(i + 1, v);
        } else if args[i].starts_with("--ui-strategies=") {
            let v = args[i][16..].to_string();
            args[i] = "--ui-strategies".to_string();
            args.insert(i + 1, v);
        }
        i += 1;
    }

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "-h" | "--help" => {
                cfg.show_help = true;
                idx += 1;
            }
            "--auto" | "auto" | "-a" => {
                // map legacy `--auto` to AutoUi (simulation via UI)
                cfg.mode = Mode::AutoUi;
                if let Some(r) = args.get(idx + 1) {
                    if let Ok(v) = r.parse::<u32>() {
                        cfg.reps = v;
                    }
                }
                if let Some(p) = args.get(idx + 2) {
                    if let Ok(v) = p.parse::<usize>() {
                        cfg.num_players = v;
                    }
                }
                idx += 3;
            }
            "--auto-ui" | "auto-ui" | "-A" => {
                cfg.mode = Mode::AutoUi;
                if let Some(r) = args.get(idx + 1) {
                    if let Ok(v) = r.parse::<u32>() {
                        cfg.reps = v;
                    }
                }
                if let Some(p) = args.get(idx + 2) {
                    if let Ok(v) = p.parse::<usize>() {
                        cfg.num_players = v;
                    }
                }
                idx += 3;
            }
            "--strategies" | "-s" => {
                if let Some(v) = args.get(idx + 1) {
                    cfg.strategies_raw = v.clone();
                    idx += 2;
                } else {
                    idx += 1;
                }
            }
            "--ui-strategies" | "-u" => {
                if let Some(v) = args.get(idx + 1) {
                    cfg.ui_str_raw = v.clone();
                    idx += 2;
                } else {
                    idx += 1;
                }
            }
            "--ui" | "ui" => {
                cfg.mode = Mode::Ui;
                idx += 1;
            }
            _ => {
                idx += 1;
            }
        }
    }

    cfg
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().skip(1).collect();
    parse_from(args)
}

pub fn print_help() {
  println!("RatJack - simulador/UI de blackjack\n");
  println!("Uso:");
  println!("  --auto-ui <reps> <players> [--strategies \"s1,s2\"]   Ejecuta simulaciones automáticas dentro de la UI");
  println!("  --ui [--ui-strategies \"s1,s2\"]                      Inicia la UI interactiva");
  println!("  -h, --help                                           Muestra esta ayuda");
  println!("\nFormato de estrategias y ejemplos: 'threshold:16,prob:0.35,random:0.5,count:hi-lo'\n");

  println!("Estrategias disponibles (formato clave:valor):");
  println!("  threshold:<N>    - Planta (stand) si el total de la mano >= N; pide (hit) en caso contrario.");
  println!("  prob:<P>         - En cada decisión, planta con probabilidad P (0.0 - 1.0).");
  println!("  random:<P>       - Similar a prob: decisión aleatoria de plantarse con probabilidad P.");
  println!("  count:<N>        - Usa conteo de cartas (p. ej. 'hi-lo') para ajustar decisiones según el conteo.");
  println!("\nNotas:");
  println!("  - Las estrategias se combinan en una lista separada por comas, p. ej. --strategies \"threshold:16,random:0.5\".");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_auto() {
        let args = vec![
            "--auto".to_string(),
            "100".to_string(),
            "3".to_string(),
            "--strategies".to_string(),
            "threshold:16,random:0.5".to_string(),
        ];
        let cfg = parse_from(args);
        assert!(matches!(cfg.mode, Mode::AutoUi));
        assert_eq!(cfg.reps, 100);
        assert_eq!(cfg.num_players, 3);
        assert_eq!(cfg.strategies_raw, "threshold:16,random:0.5");
    }

    #[test]
    fn test_parse_from_auto_ui_short() {
        let args = vec![
            "-A".to_string(),
            "50".to_string(),
            "2".to_string(),
            "--strategies=prob:0.4".to_string(),
        ];
        let cfg = parse_from(args);
        assert!(matches!(cfg.mode, Mode::AutoUi));
        assert_eq!(cfg.reps, 50);
        assert_eq!(cfg.num_players, 2);
        assert_eq!(cfg.strategies_raw, "prob:0.4");
    }

    #[test]
    fn test_parse_ui_strategies() {
        let args = vec![
            "--ui-strategies".to_string(),
            "threshold:15,prob:0.3".to_string(),
        ];
        let cfg = parse_from(args);
        assert!(matches!(cfg.mode, Mode::Ui));
        assert_eq!(cfg.ui_str_raw, "threshold:15,prob:0.3");
    }
}
