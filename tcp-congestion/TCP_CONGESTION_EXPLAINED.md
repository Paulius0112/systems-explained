# TCP Congestion Control: The Window Explained

## The Problem

Imagine you're sending files across the internet. You could send packets one at a time, waiting for each acknowledgment before sending the next. But that's painfully slow - you'd spend most of your time waiting.

The opposite extreme? Blast all your data at once. But that overwhelms the network, packets get dropped, and you end up resending everything anyway.

TCP's congestion control finds the sweet spot.

## The Congestion Window (cwnd)

The **congestion window** is simply: *how many packets can be "in flight" at once*.

"In flight" means sent but not yet acknowledged.

```
cwnd = 4 means:

[Packet 1] -----> in flight
[Packet 2] -----> in flight
[Packet 3] -----> in flight
[Packet 4] -----> in flight
[Packet 5] waiting... (can't send until one is ACK'd)
```

When an ACK comes back, a slot opens up, and you can send the next packet.

## The Two Phases

### Phase 1: Slow Start (Exponential Growth)

Despite the name, slow start is actually aggressive. The window **doubles** every round-trip:

```
Round 1: cwnd = 1  →  send 1 packet
Round 2: cwnd = 2  →  send 2 packets
Round 3: cwnd = 4  →  send 4 packets
Round 4: cwnd = 8  →  send 8 packets
Round 5: cwnd = 16 →  send 16 packets
```

Why? We don't know the network's capacity yet. Growing exponentially lets us find it quickly.

### Phase 2: Congestion Avoidance (Linear Growth)

Once cwnd reaches a threshold called **ssthresh** (slow start threshold), we switch to linear growth. The window increases by just **1 per round-trip**:

```
Round 1: cwnd = 16 → send 16 packets
Round 2: cwnd = 17 → send 17 packets
Round 3: cwnd = 18 → send 18 packets
```

Why slow down? We're getting close to the network's limit. Careful now.

## When Packets Get Lost

Packet loss = the network is congested. TCP's response:

1. **Cut ssthresh in half** (remember this pain)
2. **Reset cwnd** (back to slow start, or to ssthresh for fast recovery)
3. **Retransmit** the lost packet

```
Before loss: cwnd = 32, ssthresh = 64

  ✗ Packet lost!

After loss:  cwnd = 1, ssthresh = 16
             (start over, but now we know the limit)
```

## The Sawtooth Pattern

Over time, TCP's congestion window looks like a sawtooth:

```
cwnd
 ▲
 │      /\      /\      /\
 │     /  \    /  \    /  \
 │    /    \  /    \  /    \
 │   /      \/      \/      \
 │  /
 │ /
 └──────────────────────────────► time
   ↑        ↑       ↑
   loss     loss    loss
```

1. **Ramp up** - window grows (slow start → congestion avoidance)
2. **Drop** - packet loss detected, cut window
3. **Repeat** - continuously probing for available bandwidth

## Why This Matters

TCP congestion control is why the internet doesn't collapse. Every TCP connection is constantly:

- **Probing** for more bandwidth (growing the window)
- **Backing off** when congestion is detected (shrinking the window)
- **Sharing fairly** with other connections

It's a distributed algorithm running on billions of devices, with no central coordination, keeping the internet stable since 1988.

## The Key Insight

The congestion window is TCP's **speed dial**.

- Small window = slow and safe
- Large window = fast but risky
- TCP continuously adjusts it based on feedback (ACKs and losses)

That's it. Everything else is details.

---

*Visualize this in action: [tcp-congestion](https://github.com/...) - an animated TUI showing TCP congestion control in real-time.*
