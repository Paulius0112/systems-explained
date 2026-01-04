# TCP's Brilliant Hack: How the Internet Avoids Collapse

Every second, billions of devices blast data across the internet. They all share the same wires, the same routers, the same undersea cables. Yet somehow, it all works.

How? A 36-year-old algorithm running on every device, constantly asking: *"How fast can I go without breaking things?"*

## The Problem

Imagine a highway with no speed limits and no traffic reports. Everyone floors it. What happens?

Gridlock. Crashes. Nobody moves.

The early internet had this exact problem. Computers would send data as fast as possible, routers would overflow, packets would drop, and the whole network would grind to a halt. In 1986, the internet experienced its first "congestion collapse" - throughput dropped to a tiny fraction of capacity.

Something had to change.

## The Insight

Van Jacobson, a researcher at Lawrence Berkeley Lab, had a key insight: **packet loss is a signal**.

When a packet disappears, it means a router somewhere is overwhelmed. Instead of treating this as a failure to retry blindly, TCP could treat it as feedback: *"You're sending too fast. Slow down."*

## The Algorithm

TCP congestion control is beautifully simple. It has one variable and two rules.

**The variable: Congestion Window (cwnd)**

This is how many packets you're allowed to have "in flight" - sent but not yet acknowledged. Think of it as your speed dial.

- cwnd = 1 → Send one packet, wait for acknowledgment, send next
- cwnd = 10 → Send 10 packets, then wait
- cwnd = 100 → Send 100 packets at once

**Rule 1: When you receive an ACK, grow the window**

Acknowledgment received = the network handled it fine. Try sending a bit more.

**Rule 2: When a packet is lost, shrink the window**

Packet lost = the network is congested. Back off immediately.

That's it. Two rules. The rest is details.

## The Two Phases

### Slow Start (Not Actually Slow)

When a connection begins, you don't know the network's capacity. Could be a gigabit fiber line, could be a congested coffee shop WiFi.

So you start with cwnd = 1 and **double it every round trip**:

```
Round 1: cwnd = 1
Round 2: cwnd = 2
Round 3: cwnd = 4
Round 4: cwnd = 8
Round 5: cwnd = 16
Round 6: cwnd = 32
```

This exponential growth finds available bandwidth quickly. "Slow start" is a misnomer - it's actually aggressive. But it's slower than "just blast everything immediately."

### Congestion Avoidance (The Careful Phase)

Exponential growth can't continue forever. Once cwnd crosses a threshold (called ssthresh), TCP switches to linear growth: **add 1 per round trip**.

```
Round 1: cwnd = 16
Round 2: cwnd = 17
Round 3: cwnd = 18
Round 4: cwnd = 19
```

This is the "probing" phase. You're close to the network's limit. Increase slowly, waiting to see if something breaks.

## When Packets Drop

Eventually, you'll push too hard and lose a packet. TCP's response:

1. **Cut ssthresh in half** (remember how much was too much)
2. **Reset cwnd** (back off immediately)
3. **Start growing again** (but more carefully this time)

This creates the classic "sawtooth" pattern:

```
cwnd
  ^
  |    /\      /\      /\
  |   /  \    /  \    /  \
  |  /    \  /    \  /    \
  | /      \/      \/      \
  +--------------------------> time
         ^      ^      ^
        loss   loss   loss
```

Ramp up, hit congestion, drop back, repeat. Forever.

## Why This Works

The genius is that this algorithm is **distributed**. Every TCP connection, on every device, runs it independently. No central coordinator. No global knowledge.

Yet collectively, they share bandwidth fairly. When the network gets congested, everyone backs off. When capacity frees up, everyone gradually claims their share.

It's emergent cooperation from purely selfish rules.

## The Real-World Impact

This algorithm - with refinements over the years - handles:

- 4.5 billion internet users
- 100+ exabytes of traffic per month
- Speeds from dial-up to 400 Gbps

And it's still basically Van Jacobson's 1988 design. The core insight remains: **treat loss as a signal, and adapt.**

## Try It Yourself

I built a visualization that shows this algorithm in action. Watch the window grow during slow start, transition to congestion avoidance, then crash back down when a packet is lost.

```bash
git clone https://github.com/[your-repo]/tcp-congestion
cd tcp-congestion
cargo run --release
```

You'll see the sawtooth pattern emerge in real-time. Once you see it, you'll never forget how the internet stays alive.

---

*TCP congestion control is one of those algorithms that's invisible until you understand it - then you see it everywhere. Every download, every stream, every page load: the sawtooth is there, quietly preventing collapse.*
