use crate::animation::{CongestionPhase, TcpCongestionAnimation};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, anim: &TcpCongestionAnimation) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),   // Network animation
            Constraint::Length(6),    // Window size bar
            Constraint::Min(8),       // History graph
            Constraint::Length(3),    // Controls
        ])
        .split(frame.area());

    draw_network(frame, anim, main_chunks[0]);
    draw_window_bar(frame, anim, main_chunks[1]);
    draw_history(frame, anim, main_chunks[2]);
    draw_controls(frame, main_chunks[3]);
}

fn draw_network(frame: &mut Frame, anim: &TcpCongestionAnimation, area: Rect) {
    let block = Block::default()
        .title(" Network: Watch packets travel, ACKs return ")
        .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 40 || inner.height < 6 {
        return;
    }

    // Layout
    let sender_x = inner.x + 2;
    let receiver_x = inner.x + inner.width - 12;
    let pipe_y = inner.y + 3;

    // Sender box
    let sender = Paragraph::new(vec![
        Line::from("┌────────┐"),
        Line::from("│ SENDER │"),
        Line::from("│        │"),
        Line::from("└────────┘"),
    ]).style(Style::default().fg(Color::White));
    frame.render_widget(sender, Rect::new(sender_x, inner.y, 10, 4));

    // Receiver box
    let receiver = Paragraph::new(vec![
        Line::from("┌──────────┐"),
        Line::from("│ RECEIVER │"),
        Line::from("│          │"),
        Line::from("└──────────┘"),
    ]).style(Style::default().fg(Color::White));
    frame.render_widget(receiver, Rect::new(receiver_x, inner.y, 12, 4));

    // Draw the pipe
    let pipe_start = sender_x + 11;
    let pipe_end = receiver_x - 1;

    // Packet lane (top) - packets going right →
    let packet_label = Paragraph::new("Packets →")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(packet_label, Rect::new(pipe_start, pipe_y - 1, 10, 1));

    // ACK lane (bottom) - ACKs going left ←
    let ack_label = Paragraph::new("← ACKs")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(ack_label, Rect::new(pipe_start, pipe_y + 2, 8, 1));

    // Draw lane lines
    for x in pipe_start..pipe_end {
        let dot1 = Paragraph::new("─").style(Style::default().fg(Color::DarkGray));
        let dot2 = Paragraph::new("─").style(Style::default().fg(Color::DarkGray));
        frame.render_widget(dot1, Rect::new(x, pipe_y, 1, 1));
        frame.render_widget(dot2, Rect::new(x, pipe_y + 1, 1, 1));
    }

    let pipe_width = (pipe_end - pipe_start) as f32;

    // Draw packets in flight (going right)
    for packet in &anim.packets {
        if packet.acked || packet.x > 1.0 {
            continue;
        }

        let packet_x = pipe_start + (packet.x * pipe_width) as u16;

        if packet_x >= pipe_start && packet_x < pipe_end {
            let (symbol, color) = if packet.lost {
                ("✗", Color::Red)  // Lost packet
            } else {
                ("●", Color::Green)  // Normal packet
            };

            let marker = Paragraph::new(symbol)
                .style(Style::default().fg(color).add_modifier(Modifier::BOLD));
            frame.render_widget(marker, Rect::new(packet_x, pipe_y, 1, 1));
        }
    }

    // Draw ACKs returning (going left) - show as checkmarks
    for packet in &anim.packets {
        if !packet.acked || packet.x < 1.0 {
            continue;
        }
        // ACK travels back
        let ack_progress = (packet.x - 1.0).min(1.0);
        let ack_x = pipe_end - (ack_progress * pipe_width) as u16;

        if ack_x >= pipe_start && ack_x < pipe_end {
            let marker = Paragraph::new("✓")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            frame.render_widget(marker, Rect::new(ack_x, pipe_y + 1, 1, 1));
        }
    }

    // Stats on the right
    let stats_x = inner.x + 2;
    let stats_y = inner.y + inner.height - 3;

    let in_flight = anim.packets.iter().filter(|p| !p.acked && !p.lost && p.x <= 1.0).count();

    let stats = Line::from(vec![
        Span::styled("● ", Style::default().fg(Color::Green)),
        Span::styled("Packet   ", Style::default().fg(Color::Gray)),
        Span::styled("✓ ", Style::default().fg(Color::Cyan)),
        Span::styled("ACK   ", Style::default().fg(Color::Gray)),
        Span::styled("✗ ", Style::default().fg(Color::Red)),
        Span::styled("Lost   ", Style::default().fg(Color::Gray)),
        Span::raw("│ "),
        Span::styled(format!("In flight: {}", in_flight), Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled(format!("Sent: {}", anim.total_sent), Style::default().fg(Color::Green)),
        Span::raw("  "),
        Span::styled(format!("Lost: {}", anim.total_lost), Style::default().fg(Color::Red)),
    ]);

    let stats_para = Paragraph::new(stats);
    frame.render_widget(stats_para, Rect::new(stats_x, stats_y, inner.width - 4, 1));
}

fn draw_window_bar(frame: &mut Frame, anim: &TcpCongestionAnimation, area: Rect) {
    let block = Block::default()
        .title(" Window Size: How many packets can be in-flight at once ")
        .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 20 || inner.height < 2 {
        return;
    }

    let cwnd = anim.cwnd.floor() as usize;
    let max_display = (inner.width - 20) as usize;

    // Draw window as slots
    let bar_y = inner.y + 1;
    let bar_x = inner.x + 2;

    // Window size number
    let size_text = format!("Window = {} ", cwnd);
    let size_para = Paragraph::new(size_text)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    frame.render_widget(size_para, Rect::new(bar_x, bar_y, 12, 1));

    // Draw the bar
    let bar_start = bar_x + 12;
    for i in 0..max_display {
        let x = bar_start + i as u16;
        if i < cwnd.min(max_display) {
            let block_char = Paragraph::new("█")
                .style(Style::default().fg(Color::Green));
            frame.render_widget(block_char, Rect::new(x, bar_y, 1, 1));
        } else {
            let empty = Paragraph::new("░")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(empty, Rect::new(x, bar_y, 1, 1));
        }
    }

    // Explanation based on phase
    let explanation = match anim.phase {
        CongestionPhase::SlowStart => {
            "↑ GROWING FAST (doubles each round) - finding network capacity".to_string()
        }
        CongestionPhase::CongestionAvoidance => {
            "↗ GROWING SLOWLY (+1 per round) - being careful near limit".to_string()
        }
        CongestionPhase::FastRecovery => {
            format!("↓ PACKET LOST! Window cut in half → {:.0}", anim.cwnd)
        }
    };

    let phase_color = match anim.phase {
        CongestionPhase::SlowStart => Color::Green,
        CongestionPhase::CongestionAvoidance => Color::Yellow,
        CongestionPhase::FastRecovery => Color::Red,
    };

    let exp_para = Paragraph::new(explanation)
        .style(Style::default().fg(phase_color));
    frame.render_widget(exp_para, Rect::new(bar_x, bar_y + 2, inner.width - 4, 1));
}

fn draw_history(frame: &mut Frame, anim: &TcpCongestionAnimation, area: Rect) {
    let block = Block::default()
        .title(" History: Watch the 'sawtooth' pattern emerge ")
        .title_style(Style::default().fg(Color::Cyan))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 10 || inner.height < 4 {
        return;
    }

    let history = &anim.cwnd_history;
    if history.is_empty() {
        return;
    }

    let graph_height = (inner.height - 2) as f32;
    let graph_width = (inner.width - 4) as usize;
    let max_cwnd = anim.max_cwnd_in_history().max(16.0);

    let start_idx = history.len().saturating_sub(graph_width);
    let graph_x = inner.x + 3;
    let graph_y_bottom = inner.y + inner.height - 2;

    // Y-axis labels
    let max_label = format!("{:.0}", max_cwnd);
    let max_para = Paragraph::new(max_label).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(max_para, Rect::new(inner.x, inner.y + 1, 3, 1));

    let zero = Paragraph::new("0").style(Style::default().fg(Color::DarkGray));
    frame.render_widget(zero, Rect::new(inner.x + 1, graph_y_bottom, 1, 1));

    // Draw the graph - all in green for simplicity
    for (i, point) in history[start_idx..].iter().enumerate() {
        if i >= graph_width {
            break;
        }
        let x = graph_x + i as u16;
        let normalized = (point.cwnd / max_cwnd).min(1.0);
        let y = graph_y_bottom - (normalized * graph_height) as u16;

        if y >= inner.y && y <= graph_y_bottom {
            // Use green for growing, red for the drop
            let color = if i > 0 {
                let prev_idx = start_idx + i - 1;
                if prev_idx < history.len() && history[prev_idx].cwnd > point.cwnd + 0.5 {
                    Color::Red  // Dropping
                } else {
                    Color::Green  // Growing
                }
            } else {
                Color::Green
            };

            let marker = Paragraph::new("█").style(Style::default().fg(color));
            frame.render_widget(marker, Rect::new(x, y, 1, 1));
        }
    }

    // Explanation
    let exp_y = inner.y + inner.height - 1;
    let explanation = Paragraph::new("Green = window growing    Red = packet lost, window cut in half")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(explanation, Rect::new(inner.x + 2, exp_y, inner.width - 4, 1));
}

fn draw_controls(frame: &mut Frame, area: Rect) {
    let text = Line::from(vec![
        Span::styled(" q ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::styled(" Quit   ", Style::default().fg(Color::Gray)),
        Span::styled(" r ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::styled(" Reset   ", Style::default().fg(Color::Gray)),
        Span::raw("│ "),
        Span::styled("Each ● is a packet. Watch them travel, get ACKed (✓), and see the window grow!", Style::default().fg(Color::DarkGray)),
    ]);

    let para = Paragraph::new(text);
    frame.render_widget(para, Rect::new(area.x + 1, area.y + 1, area.width - 2, 1));
}
