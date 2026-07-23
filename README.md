# Laboratorio 2: Game of Life

Implementación de Game Of Life de Conway en Rust. El programa utiliza un framebuffer con Raylib para mostrar la evolución de diferentes patrones de células.

## Ejecución

Se necesita tener instalados Rust y Cargo.

Comando para correrlo:
```bash
cargo run
```

## Controles

| Tecla | Acción |
| --- | --- |
| `Espacio` | Pausar o reanudar la simulación |
| `+` | Aumentar la velocidad |
| `-` | Disminuir la velocidad |
| `R` | Reiniciar la simulación |
| `S` | Guardar una captura de la generación actual |
| `Esc` | Cerrar el programa |

## Patrones

La configuración inicial combina pulsares, planeadores, naves espaciales, cañones de planeadores, acorns y osciladores. Los patrones están orientados para desplazarse, evolucionar y colisionar en diferentes zonas del tablero.

## Demostración

![Demo](assets/game-of-life.gif)