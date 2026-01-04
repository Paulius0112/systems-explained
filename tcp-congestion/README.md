# TCP Congestion Control Visualization

Animated TUI visualization of TCP congestion control algorithms (Reno-style).

## Overview

This tool visualizes TCP's congestion control mechanism in real-time:

| Phase | Description | cwnd Growth |
|-------|-------------|-------------|
| **Slow Start** | Initial phase, aggressive growth | Exponential (+1 per ACK) |
| **Congestion Avoidance** | After ssthresh, conservative growth | Linear (+1/cwnd per ACK) |
| **Fast Recovery** | After triple duplicate ACK | Halve cwnd, skip slow start |

## Features

- Real-time cwnd (congestion window) graph
- ssthresh (slow start threshold) visualization
- Animated packet flow between sender and receiver
- Packet loss simulation with retransmission
- Event log showing ACKs, losses, and phase changes
- Statistics tracking (sent, acked, lost, retransmits)

## Requirements

- Rust 1.70+

## Usage

```bash
cargo run --release
```

### Controls

- `q` or `Esc` - Quit
- `r` - Reset simulation

## How It Works

The visualization shows:

1. **Graph Panel**: cwnd over time with ssthresh line
   - Green = Slow Start (exponential growth)
   - Cyan = Congestion Avoidance (linear growth)
   - Yellow = Fast Recovery
   - Red line = ssthresh threshold

2. **Packet Animation**: Packets traveling from sender to receiver
   - Blue dots = Normal packets
   - Yellow = Retransmitted packets
   - Red X = Lost packets

3. **Statistics**: Packet counts and loss rate

4. **Events**: Real-time log of network events

## TCP Congestion Control Algorithm

```
On ACK received:
  if cwnd < ssthresh:
    cwnd += 1              # Slow Start: exponential
  else:
    cwnd += 1/cwnd         # Congestion Avoidance: linear

On packet loss (timeout):
  ssthresh = cwnd / 2
  cwnd = 1                 # Reset to slow start

On triple duplicate ACK:
  ssthresh = cwnd / 2
  cwnd = ssthresh + 3      # Fast Recovery
```

## License

MIT
