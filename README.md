Juego de Pelota RT is a continuation of my DEVS exploration in Rust — this time, the focus is on real-time simulation on an embedded platform (ESP32-C6).

Project Overview
Unlike the first version (juego_de_pelota, which only runs on PC), this project is designed to compile and run directly on the ESP32-C6 board.
It features different test examples that aim to:
Explore real-time behavior using the DEVS formalism.
Test how models interact under timing constraints.

Implementation Notes
Built using the xdevs crate for DEVS modeling in Rust.
Several experiments were conducted using wait_until() and other timing strategies.
Eventually, it was confirmed that xdevs already provides solid support for real-time simulation, which simplified the implementation.

Goals
Validate how DEVS models behave in embedded real-time environments.
Prepare the foundation for future embedded control systems (e.g. for robotics or satellite simulation).
