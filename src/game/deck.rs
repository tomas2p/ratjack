use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub enum Palo {
    Corazones,
    Diamantes,
    Tréboles,
    Picas,
}

#[derive(Debug, Clone, Copy)]
pub struct Carta {
    pub valor: u8,
    pub palo: Palo,
}

impl Carta {
    pub fn simbolo(&self) -> char {
        match self.palo {
            Palo::Corazones => '♡',
            Palo::Diamantes => '♢',
            Palo::Tréboles => '♧',
            Palo::Picas => '♤',
        }
    }

    pub fn puntos(&self) -> u8 {
        match self.valor {
            1 => 11,
            11 | 12 | 13 => 10,
            _ => self.valor,
        }
    }

    pub fn valor_str(&self) -> String {
        match self.valor {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => self.valor.to_string(),
        }
    }
}

// Función para crear una baraja nueva barajada
pub fn crear_baraja() -> Vec<Carta> {
    let palos = [
        Palo::Corazones,
        Palo::Diamantes,
        Palo::Tréboles,
        Palo::Picas,
    ];
    let mut baraja = Vec::new();

    for &palo in &palos {
        for valor in 1..=13 {
            baraja.push(Carta { valor, palo });
        }
    }

    baraja.shuffle(&mut rand::thread_rng());
    baraja
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_baraja_length() {
        let baraja = crear_baraja();
        assert_eq!(baraja.len(), 52);
    }

    #[test]
    fn test_crear_baraja_unique_cards() {
        let baraja = crear_baraja();
        let mut unique_cards = Vec::new();

        for card in &baraja {
            if !unique_cards.iter().any(|c: &Carta| {
                c.valor == card.valor
                    && std::mem::discriminant(&c.palo) == std::mem::discriminant(&card.palo)
            }) {
                unique_cards.push(*card);
            }
        }

        assert_eq!(unique_cards.len(), 52);
    }

    #[test]
    fn test_carta_simbolo() {
        let carta_corazones = Carta {
            valor: 5,
            palo: Palo::Corazones,
        };
        let carta_diamantes = Carta {
            valor: 10,
            palo: Palo::Diamantes,
        };
        let carta_treboles = Carta {
            valor: 1,
            palo: Palo::Tréboles,
        };
        let carta_picas = Carta {
            valor: 12,
            palo: Palo::Picas,
        };

        assert_eq!(carta_corazones.simbolo(), '♡');
        assert_eq!(carta_diamantes.simbolo(), '♢');
        assert_eq!(carta_treboles.simbolo(), '♧');
        assert_eq!(carta_picas.simbolo(), '♤');
    }

    #[test]
    fn test_carta_puntos() {
        assert_eq!(
            Carta {
                valor: 1,
                palo: Palo::Corazones
            }
            .puntos(),
            11
        ); // As
        assert_eq!(
            Carta {
                valor: 5,
                palo: Palo::Diamantes
            }
            .puntos(),
            5
        ); // Número
        assert_eq!(
            Carta {
                valor: 10,
                palo: Palo::Tréboles
            }
            .puntos(),
            10
        ); // Número 10
        assert_eq!(
            Carta {
                valor: 11,
                palo: Palo::Picas
            }
            .puntos(),
            10
        ); // J
        assert_eq!(
            Carta {
                valor: 12,
                palo: Palo::Corazones
            }
            .puntos(),
            10
        ); // Q
        assert_eq!(
            Carta {
                valor: 13,
                palo: Palo::Diamantes
            }
            .puntos(),
            10
        ); // K
    }

    #[test]
    fn test_carta_valor_str() {
        assert_eq!(
            Carta {
                valor: 1,
                palo: Palo::Corazones
            }
            .valor_str(),
            "A"
        );
        assert_eq!(
            Carta {
                valor: 2,
                palo: Palo::Diamantes
            }
            .valor_str(),
            "2"
        );
        assert_eq!(
            Carta {
                valor: 10,
                palo: Palo::Tréboles
            }
            .valor_str(),
            "10"
        );
        assert_eq!(
            Carta {
                valor: 11,
                palo: Palo::Picas
            }
            .valor_str(),
            "J"
        );
        assert_eq!(
            Carta {
                valor: 12,
                palo: Palo::Corazones
            }
            .valor_str(),
            "Q"
        );
        assert_eq!(
            Carta {
                valor: 13,
                palo: Palo::Diamantes
            }
            .valor_str(),
            "K"
        );
    }
}
