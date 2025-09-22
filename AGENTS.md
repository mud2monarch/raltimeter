# AGENTS

## Background
I'm starting my first embedded Rust project focused on the ESP32-C6 chipset. The goal is to build an altimeter that reads barometric pressure, allows altitude calibration via a rotary dial input, and displays the computed altitude on an e-ink display.

## Key Components
- ESP32-C6 microcontroller
- Barometric pressure sensor for altitude measurements
- Rotary dial for user-supplied calibration input
- E-ink display for presenting altitude readings

## High-Level Objectives
- Bring up the ESP32-C6 in the embedded Rust environment and establish reliable communication with the barometric sensor.
- Accept user calibration input through the rotary dial and apply it to altitude calculations.
- Render altitude data clearly and efficiently on the e-ink display.

## Guiding Principles
- Favor safe abstractions in Rust while keeping close to the hardware when necessary.
- Iterate with small, testable hardware and firmware milestones.
- Prioritize power efficiency and readability on the e-ink display.
