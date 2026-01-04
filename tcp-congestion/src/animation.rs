use ratatui::style::Color;

/// TCP Congestion Control Phase
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CongestionPhase {
    SlowStart,
    CongestionAvoidance,
    FastRecovery,
}

impl CongestionPhase {
    pub fn name(&self) -> &'static str {
        match self {
            CongestionPhase::SlowStart => "SLOW START",
            CongestionPhase::CongestionAvoidance => "CONGESTION AVOIDANCE",
            CongestionPhase::FastRecovery => "FAST RECOVERY",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            CongestionPhase::SlowStart => Color::Green,
            CongestionPhase::CongestionAvoidance => Color::Cyan,
            CongestionPhase::FastRecovery => Color::Yellow,
        }
    }
}

/// A single packet being transmitted
#[derive(Debug, Clone)]
pub struct Packet {
    pub seq_num: u32,
    pub x: f32,           // 0.0 = sender, 1.0 = receiver
    pub acked: bool,
    pub lost: bool,
    pub retransmit: bool,
}

/// Event that occurred during simulation
#[derive(Debug, Clone)]
pub struct Event {
    pub tick: u32,
    pub event_type: EventType,
}

#[derive(Debug, Clone)]
pub enum EventType {
    PacketSent(u32),
    AckReceived(u32),
    PacketLost(u32),
    TripleDupAck,
    Timeout,
    CwndChanged(f32),
    SsthreshChanged(f32),
    PhaseChanged(CongestionPhase),
}

/// History point for the cwnd graph
#[derive(Debug, Clone, Copy)]
pub struct CwndPoint {
    pub tick: u32,
    pub cwnd: f32,
    pub ssthresh: f32,
    pub phase: CongestionPhase,
}

/// Main TCP Congestion Control Animation
#[derive(Debug)]
pub struct TcpCongestionAnimation {
    // Congestion control state
    pub cwnd: f32,              // Congestion window (in MSS)
    pub ssthresh: f32,          // Slow start threshold
    pub phase: CongestionPhase,

    // Packet tracking
    pub packets: Vec<Packet>,
    pub next_seq: u32,
    pub last_acked: u32,
    pub dup_ack_count: u32,

    // History for graphing
    pub cwnd_history: Vec<CwndPoint>,
    pub events: Vec<Event>,

    // Animation timing
    pub tick: u32,
    ticks_since_send: u32,
    ticks_since_loss: u32,
    loss_scheduled_at: Option<u32>,

    // Stats
    pub total_sent: u32,
    pub total_acked: u32,
    pub total_lost: u32,
    pub total_retransmits: u32,
}

impl TcpCongestionAnimation {
    pub fn new() -> Self {
        let initial_ssthresh = 16.0;
        let mut anim = Self {
            cwnd: 1.0,
            ssthresh: initial_ssthresh,
            phase: CongestionPhase::SlowStart,
            packets: Vec::new(),
            next_seq: 1,
            last_acked: 0,
            dup_ack_count: 0,
            cwnd_history: Vec::new(),
            events: Vec::new(),
            tick: 0,
            ticks_since_send: 0,
            ticks_since_loss: 0,
            loss_scheduled_at: Some(600), // Schedule first loss
            total_sent: 0,
            total_acked: 0,
            total_lost: 0,
            total_retransmits: 0,
        };

        // Record initial state
        anim.record_cwnd();
        anim
    }

    fn record_cwnd(&mut self) {
        self.cwnd_history.push(CwndPoint {
            tick: self.tick,
            cwnd: self.cwnd,
            ssthresh: self.ssthresh,
            phase: self.phase,
        });

        // Keep history bounded
        if self.cwnd_history.len() > 500 {
            self.cwnd_history.remove(0);
        }
    }

    fn add_event(&mut self, event_type: EventType) {
        self.events.push(Event {
            tick: self.tick,
            event_type,
        });

        // Keep events bounded
        if self.events.len() > 50 {
            self.events.remove(0);
        }
    }

    pub fn update(&mut self) {
        self.tick += 1;
        self.ticks_since_send += 1;
        self.ticks_since_loss += 1;

        // Move packets
        self.update_packets();

        // Check for scheduled loss
        if let Some(loss_tick) = self.loss_scheduled_at {
            if self.tick >= loss_tick && !self.packets.is_empty() {
                self.trigger_packet_loss();
                self.loss_scheduled_at = None;
            }
        }

        // Send new packets based on cwnd
        let send_interval = 50; // Ticks between sends (slower for readability)
        if self.ticks_since_send >= send_interval {
            let in_flight = self.packets.iter().filter(|p| !p.acked && !p.lost).count() as f32;

            if in_flight < self.cwnd {
                self.send_packet();
                self.ticks_since_send = 0;
            }
        }

        // Process ACKs for packets that reached the receiver
        self.process_acks();

        // Schedule next loss event periodically
        if self.loss_scheduled_at.is_none() && self.ticks_since_loss > 800 {
            self.loss_scheduled_at = Some(self.tick + 400 + (self.tick % 200));
        }
    }

    fn send_packet(&mut self) {
        let packet = Packet {
            seq_num: self.next_seq,
            x: 0.0,
            acked: false,
            lost: false,
            retransmit: false,
        };

        self.add_event(EventType::PacketSent(self.next_seq));
        self.packets.push(packet);
        self.next_seq += 1;
        self.total_sent += 1;
    }

    fn update_packets(&mut self) {
        let speed = 0.008; // Movement speed

        for packet in &mut self.packets {
            if !packet.lost {
                packet.x += speed;
            }
        }

        // Remove packets after ACK has "traveled back" (x > 2.0 means full round trip)
        // Also remove lost packets after they've been visible for a bit
        self.packets.retain(|p| p.x < 2.0);
    }

    fn process_acks(&mut self) {
        let mut new_acks = Vec::new();

        for packet in &mut self.packets {
            // Packet reached receiver
            if packet.x >= 1.0 && !packet.acked && !packet.lost {
                packet.acked = true;
                new_acks.push(packet.seq_num);
            }
        }

        for seq in new_acks {
            self.handle_ack(seq);
        }
    }

    fn handle_ack(&mut self, seq_num: u32) {
        self.add_event(EventType::AckReceived(seq_num));
        self.total_acked += 1;

        if seq_num > self.last_acked {
            // New ACK
            self.last_acked = seq_num;
            self.dup_ack_count = 0;

            // Grow cwnd based on phase
            let old_cwnd = self.cwnd;
            match self.phase {
                CongestionPhase::SlowStart => {
                    // Exponential growth: cwnd += 1 for each ACK
                    self.cwnd += 1.0;

                    // Check if we should transition to congestion avoidance
                    if self.cwnd >= self.ssthresh {
                        self.phase = CongestionPhase::CongestionAvoidance;
                        self.add_event(EventType::PhaseChanged(self.phase));
                    }
                }
                CongestionPhase::CongestionAvoidance => {
                    // Linear growth: cwnd += 1/cwnd for each ACK
                    self.cwnd += 1.0 / self.cwnd;
                }
                CongestionPhase::FastRecovery => {
                    // Exit fast recovery
                    self.cwnd = self.ssthresh;
                    self.phase = CongestionPhase::CongestionAvoidance;
                    self.add_event(EventType::PhaseChanged(self.phase));
                }
            }

            if (self.cwnd - old_cwnd).abs() > 0.01 {
                self.add_event(EventType::CwndChanged(self.cwnd));
                self.record_cwnd();
            }
        } else {
            // Duplicate ACK
            self.dup_ack_count += 1;

            if self.dup_ack_count == 3 {
                self.handle_triple_dup_ack();
            }
        }
    }

    fn trigger_packet_loss(&mut self) {
        // Find a packet in flight to mark as lost
        let mut lost_seq = None;
        for packet in &mut self.packets {
            if !packet.acked && !packet.lost && packet.x > 0.3 && packet.x < 0.7 {
                packet.lost = true;
                lost_seq = Some(packet.seq_num);
                break;
            }
        }

        if let Some(seq) = lost_seq {
            self.total_lost += 1;
            self.add_event(EventType::PacketLost(seq));
            self.ticks_since_loss = 0;

            // Trigger timeout-based recovery
            self.handle_timeout();
        }
    }

    fn handle_triple_dup_ack(&mut self) {
        self.add_event(EventType::TripleDupAck);

        // Fast retransmit / fast recovery
        self.ssthresh = (self.cwnd / 2.0).max(2.0);
        self.cwnd = self.ssthresh + 3.0;
        self.phase = CongestionPhase::FastRecovery;

        self.add_event(EventType::SsthreshChanged(self.ssthresh));
        self.add_event(EventType::CwndChanged(self.cwnd));
        self.add_event(EventType::PhaseChanged(self.phase));
        self.record_cwnd();

        // Retransmit lost packet
        self.retransmit_packet();
    }

    fn handle_timeout(&mut self) {
        self.add_event(EventType::Timeout);

        // Cut window in half (simplified for educational purposes)
        self.ssthresh = (self.cwnd / 2.0).max(2.0);
        self.cwnd = self.ssthresh;  // Cut in half
        self.phase = CongestionPhase::FastRecovery;
        self.dup_ack_count = 0;

        self.add_event(EventType::SsthreshChanged(self.ssthresh));
        self.add_event(EventType::CwndChanged(self.cwnd));
        self.add_event(EventType::PhaseChanged(self.phase));
        self.record_cwnd();

        // Retransmit
        self.retransmit_packet();
    }

    fn retransmit_packet(&mut self) {
        // Find first lost packet and retransmit
        let lost_seq = self.packets.iter()
            .find(|p| p.lost)
            .map(|p| p.seq_num);

        if let Some(seq) = lost_seq {
            // Remove the lost packet
            self.packets.retain(|p| p.seq_num != seq);

            // Create retransmit
            let packet = Packet {
                seq_num: seq,
                x: 0.0,
                acked: false,
                lost: false,
                retransmit: true,
            };
            self.packets.push(packet);
            self.total_retransmits += 1;
        }
    }

    pub fn max_cwnd_in_history(&self) -> f32 {
        self.cwnd_history.iter()
            .map(|p| p.cwnd.max(p.ssthresh))
            .fold(16.0_f32, |a, b| a.max(b))
    }
}

impl Default for TcpCongestionAnimation {
    fn default() -> Self {
        Self::new()
    }
}
