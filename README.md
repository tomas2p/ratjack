![](ratjack-logo.PNG)
# 🃏 RatJack - Blackjack en Rust con Ratatui

**RatJack** es un juego de **Blackjack** interactivo y simulador escrito en **Rust**, con una interfaz de usuario basada en **Ratatui** para la terminal.

## 🎲 Características y Mejoras Recientes

✅ **Interfaz Visual de Cartas Mejorada:** Las cartas se renderizan lado a lado en filas ordenadas con colores según su palo (rojo para corazones y diamantes, cyan/blanco para picas y tréboles), evitando recortes verticales sin importar cuántas cartas se acumulen en la mano.  
✅ **Conteo de Cartas de la Banca:** Durante tu turno, se muestra cuántas cartas tiene la banca en total (ej. `? (2 cartas)`) junto con marcadores visuales para las cartas ocultas, respetando las reglas del Blackjack.  
✅ **Victoria Automática al 21:** Al alcanzar 21 puntos (al repartir o pedir), el turno de la banca se resuelve automáticamente sin requerir pulsar ninguna tecla.  
✅ **Continuación Inmediata al Plantarse:** Al elegir plantarse (`s` o `2`), la partida continúa y se resuelve de forma instantánea.  
✅ **Estrategias Personalizables:** Configura algoritmos separados para la banca y el jugador en la UI.

---

## 📊 Comparativa de Estrategias

RatJack incluye un motor de estrategias automatizadas que puedes configurar mediante `--ui-strategies` o `--strategies`. Aquí tienes la comparativa de cada tipo:

| Estrategia | Formato de Ejemplo | Descripción | Comportamiento Típico |
| :--- | :--- | :--- | :--- |
| **Threshold** (Umbral) | `threshold:17` | Se planta (stand) si la puntuación es mayor o igual al valor indicado; de lo contrario, pide (hit). | Simple, determinista y directa. Es la regla estándar usada por la banca (por defecto 17). |
| **Random** (Aleatoria) | `random:0.5` | Toma decisiones de pedir carta basadas en una probabilidad aleatoria constante $P$. | Impredecible; útil para probar caos o comportamientos erráticos. |
| **Prob** (Probabilidad de Bust) | `prob:0.4` | Simula las cartas restantes del mazo para calcular la probabilidad de pasarse (*bust*) antes de decidir. | Conservadora o calculadora; toma decisiones basadas en el riesgo matemático del mazo. |
| **Count** (Conteo Hi-Lo) | `count:17` | Utiliza el sistema clásico de conteo de cartas Hi-Lo para ajustar el umbral según la baraja restante. | Estrategia avanzada de conteo de cartas para maximizar ventaja estadística. |

---

## 🚀 Instalación y Ejecución

Asegúrate de tener **Rust** y **Cargo** instalados. Clona el repositorio y ejecútalo con:

```bash
git clone https://github.com/tomas2p/ratjack.git
cd ratjack
cargo run -- --ui --ui-strategies "threshold:17,threshold:16"
```

## 🎮 Cómo jugar

Usa las teclas en el modo interactivo:

- <kbd>↵ (Enter)</kbd> / <kbd>1</kbd> / <kbd>p</kbd> → Pedir carta (Hit)
- <kbd>2</kbd> / <kbd>s</kbd> → Plantarse (Stand)
- <kbd>n</kbd> → Nueva partida
- <kbd>q</kbd> → Salir

## 📜 Licencia

Este proyecto está bajo la licencia [MIT](LICENSE). ¡Siéntete libre de contribuir!
