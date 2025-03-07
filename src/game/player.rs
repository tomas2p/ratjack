use crate::game::deck::Carta;

pub struct Jugador {
    pub mano: Vec<Carta>,
    pub nombre: String,
    pub puntos: u8,
    pub partidas_ganadas: u32,
}

impl Jugador {
    pub fn nuevo() -> Self {
        Jugador {
            mano: Vec::new(),
            nombre: "Jugador".to_string(),
            puntos: 0,
            partidas_ganadas: 0,
        }
    }

    pub fn tomar_carta(&mut self, baraja: &mut Vec<Carta>) {
        if let Some(carta) = baraja.pop() {
            self.mano.push(carta);
        }
    }

    pub fn puntaje(&self) -> u8 {
        let mut total = 0;
        let mut ases = 0;

        for carta in &self.mano {
            total += carta.puntos();
            if carta.valor == 1 {
                ases += 1;
            }
        }

        while total > 21 && ases > 0 {
            total -= 10;
            ases -= 1;
        }

        total
    }

    pub fn partida_ganada(&mut self) {
        self.partidas_ganadas += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::{Carta, Palo};

    #[test]
    fn test_jugador_nuevo() {
        let jugador = Jugador::nuevo();
        assert_eq!(jugador.nombre, "Jugador");
        assert_eq!(jugador.puntos, 0);
        assert_eq!(jugador.partidas_ganadas, 0);
        assert!(jugador.mano.is_empty());
    }

    #[test]
    fn test_tomar_carta() {
        let mut jugador = Jugador::nuevo();
        let mut baraja = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 10,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 5,
            },
        ];

        jugador.tomar_carta(&mut baraja);
        assert_eq!(jugador.mano.len(), 1);
        assert_eq!(baraja.len(), 1);
        assert_eq!(jugador.mano[0].valor, 5);

        jugador.tomar_carta(&mut baraja);
        assert_eq!(jugador.mano.len(), 2);
        assert!(baraja.is_empty());
    }

    #[test]
    fn test_puntaje_simple() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 10,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 5,
            },
        ];

        assert_eq!(jugador.puntaje(), 15);
    }

    #[test]
    fn test_puntaje_con_as() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            }, // As
            Carta {
                palo: Palo::Diamantes,
                valor: 10,
            }, // 10
        ];

        assert_eq!(jugador.puntaje(), 21);
    }

    #[test]
    fn test_puntaje_con_multiples_ases() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            }, // As
            Carta {
                palo: Palo::Diamantes,
                valor: 1,
            }, // As
            Carta {
                palo: Palo::Tréboles,
                valor: 1,
            }, // As
        ];

        assert_eq!(jugador.puntaje(), 13); // 11 + 1 + 1
    }

    #[test]
    fn test_puntaje_con_blackjack() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            }, // As
            Carta {
                palo: Palo::Diamantes,
                valor: 13,
            }, // Rey (10 puntos)
        ];

        assert_eq!(jugador.puntaje(), 21);
    }

    #[test]
    fn test_partida_ganada() {
        let mut jugador = Jugador::nuevo();
        assert_eq!(jugador.partidas_ganadas, 0);

        jugador.partida_ganada();
        assert_eq!(jugador.partidas_ganadas, 1);

        jugador.partida_ganada();
        assert_eq!(jugador.partidas_ganadas, 2);
    }

    #[test]
    fn test_puntaje_as_que_cambia() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            }, // As
            Carta {
                palo: Palo::Diamantes,
                valor: 5,
            }, // 5
            Carta {
                palo: Palo::Tréboles,
                valor: 10,
            }, // 10
        ];

        assert_eq!(jugador.puntaje(), 16); // As vale 1
    }

    #[test]
    fn test_puntaje_con_figuras() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 11,
            }, // J
            Carta {
                palo: Palo::Diamantes,
                valor: 12,
            }, // Q
        ];

        assert_eq!(jugador.puntaje(), 20);
    }

    #[test]
    fn test_puntaje_con_mas_de_21() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 10,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 10,
            },
            Carta {
                palo: Palo::Tréboles,
                valor: 5,
            },
        ];

        assert_eq!(jugador.puntaje(), 25);
    }
}
