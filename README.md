# Systems Explained

Animated TUI visualizations that explain how computer systems and networking protocols work.

## Visualizations

| Topic | Description |
|-------|-------------|
| [tcp-congestion](./tcp-congestion) | TCP congestion control - watch the "sawtooth" pattern emerge |

## Requirements

- Rust 1.70+

## Usage

Each folder is a standalone Rust project:

```bash
cd tcp-congestion
cargo run --release
```

## Recording

Each project includes a `record.sh` script to capture a GIF demo.
