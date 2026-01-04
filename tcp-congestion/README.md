# TCP Congestion Control Visualization

Animated TUI visualization of TCP congestion control - watch the "sawtooth" pattern emerge.

![Demo](demo.gif)

## Overview

TCP congestion control has one variable and two rules:

**The variable:** Congestion Window (cwnd) - how many packets can be "in flight"

**The rules:**
- ACK received → window grows
- Packet lost → window cut in half

## Usage

```bash
cargo run --release
```

### Controls

- `q` or `Esc` - Quit
- `r` - Reset simulation

## The Phases

| Phase | What happens | Window growth |
|-------|--------------|---------------|
| **Slow Start** | Finding network capacity | Doubles each round (1→2→4→8→16) |
| **Congestion Avoidance** | Being careful near limit | +1 per round (16→17→18→19) |
| **Recovery** | Packet lost! | Cut in half (16→8) |

## Recording

To record a new demo GIF:

```bash
./record.sh
```

Requires [asciinema](https://asciinema.org/) and [agg](https://github.com/asciinema/agg).
