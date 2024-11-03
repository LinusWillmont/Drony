# Drony

A DIY drone project.

## Components

- 1x Arduino Nano
- 1x L239D Motor Driver
- 1x DC 5V Motor
- 1x Joystick

## Wiring Scheme

TODO: Add wiring schematic here.

## Code

This project is developed using Rust. For guidance on Rust programming,
refer to the [Rust Embedded Book](https://docs.rust-embedded.org/book/intro/index.html).

### Flashing the Arduino

To flash the Arduino, you'll need to use the `ravedude` tool. Follow these steps:

1. Set the environment variable for the USB port:

   ```bash
   export RAVEDUDE_PORT=/dev/ttyUSB0
   ```

2. Run the project with Cargo:

   ```bash
   cargo run
   ```
