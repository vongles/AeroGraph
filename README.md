      _                     ____ GRAPH 
     / \   ___  _ __ ___   / ___|_ __ __ _ _ __ | |__ 
    / _ \ / _ \| '__/ _ \ | |  _| '__/ _` | '_ \| '_ \ 
   / ___ \  __/| | | (_) || |_| | | | (_| | |_) | | | |
  /_/   \_\___||_|  \___/  \____|_|  \__,_| .__/|_| |_|
                                          |_|           
  MASSIVELY PARALLEL 6-DOF ASYNC VECTOR-FIELD ENGINE

---

### System Architecture

AeroGraph is an Entity-Aspect-Component / Lock-Free Actor DAG Physics Engine designed for ultra-high-fidelity simulation of complex physical phenomena—where secondary and tertiary physical effects (EMI, mechanical harmonics, thermal drag) emerge naturally without hardcoded boolean triggers.

                   ┌──────────────────────────────────────┐
                   │        AEROGRAPH CORE ENGINE         │
                   │    (Lock-Free Async DAG Scheduler)   │
                   └──────────────────┬───────────────────┘
                                      │
        ┌─────────────────────────────┴─────────────────────────────┐
        ▼                                                           ▼
┌───────────────────────────────┐               ┌───────────────────────────────┐
│     AEROGRAPH STUDIO (GUI)    │               │    PY-AEROGRAPH (AI GYM)      │
├───────────────────────────────┤               ├───────────────────────────────┤
│ • Node-Based Visual Pipeline  │               │ • PyTorch/NumPy Zero-Copy Gym │
│ • 3D Wgpu Spatial Field View  │               │ • 10,000+ Parallel Vector Sim │
│ • Real-time Telemetry & HIL   │               │ • Native PyO3 Rust Bindings   │
└───────────────────────────────┘               └───────────────────────────────┘

---

### Workspace Structure

* aerograph-core: Headless physics engine, lock-free SPSC queues, 6-DoF rigid body kinematics, aspect modules, and wgpu/rayon vector-field compute.
* aerograph-gui: Visual node editor and real-time 3D vector-field preview powered by egui and wgpu.
* py-aerograph: High-performance Rust-to-Python bindings for RL training loops via PyO3 and maturin.

---

### Quickstart

Build Physics Core & Run Tests
$ cargo test -p aerograph-core

Launch AeroGraph Studio GUI
$ cargo run -p aerograph-gui

Build Python RL Bindings
$ cd crates/py-aerograph
$ maturin develop --release

---

### License

Copyright (c) 2026 AeroGraph Development Team. All Rights Reserved.  
Proprietary License - See LICENSE for details.
