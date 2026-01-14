use crate::app::{App, Mode};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    match app.mode {
        Mode::Menu => render_menu(f, app, area),
        Mode::Writing => render_writing(f, app, area),
        Mode::Flow => render_flow(f, app, area),
        Mode::FlowHistory => render_history(f, app, area),
        Mode::Settings => render_settings(f, app, area),
        Mode::Drafts => render_drafts(f, app, area),
    }

    // Overlay message
    if let Some(msg) = &app.message {
        let msg_len = msg.len() as u16 + 4;
        let msg_rect = Rect::new(area.width - msg_len, area.height - 1, msg_len, 1);
        let p = Paragraph::new(msg.as_str())
            .style(Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD));
        f.render_widget(p, msg_rect);
    }
}

fn render_menu(f: &mut Frame, _app: &App, area: Rect) {
    let output = vec![
        Line::from(vec![Span::raw(" writeapp ").bold()]),
        Line::from(""),
        Line::from(" [n] New Draft"),
        Line::from(" [f] Flow Mode (10 min)"),
        Line::from(" [5] Flow Mode (5 min)"),
        Line::from(" [1] Flow Mode (15 min)"),
        Line::from(" [h] History"),
        Line::from(" [d] Drafts"),
        Line::from(" [s] Settings"),
        Line::from(" [q] Quit"),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Menu ");
    let p = Paragraph::new(output).block(block);
    f.render_widget(p, area);
}

fn render_writing(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    if app.preview_mode_active {
         let text_content = app.textarea.lines().join("\n");
         let formatted_lines = parse_markdown_to_lines(&text_content); 
         
         let block = Block::default().borders(Borders::ALL).title(" Preview (Markdown Read Only) ");
         let p = Paragraph::new(formatted_lines)
            .wrap(ratatui::widgets::Wrap { trim: false })
            .block(block);
         f.render_widget(p, chunks[0]);
         
    } else {
        // Edit Mode
        app.textarea.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title(" Writing ")
        );
        // Use Focus Mode styles if active
        if app.focus_mode_active {
            app.textarea.set_style(Style::default().fg(Color::DarkGray));
            app.textarea.set_cursor_line_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
        } else {
            app.textarea.set_style(Style::default());
            app.textarea.set_cursor_line_style(Style::default()); 
        }
        
        f.render_widget(&app.textarea, chunks[0]);
    }

    let count = app.textarea.lines().join(" ").split_whitespace().count();
    let status = format!(" Words: {} | Esc: Menu | Ctrl+S: Save | Ctrl+F: Focus | Ctrl+P: Preview", count);
    f.render_widget(Paragraph::new(status).style(Style::default().fg(Color::DarkGray)), chunks[1]);
}

fn render_flow(f: &mut Frame, app: &mut App, area: Rect) {
    // Calculate a centered text area with a max width (e.g. 100 chars)
    // This adds large side margins on wide screens for a better reading experience
    let max_width = 100u16;
    let target_width = if area.width > max_width { max_width } else { area.width };
    let horizontal_padding = (area.width.saturating_sub(target_width)) / 2;
    
    // Define the centered area
    let text_area = Rect {
        x: area.x + horizontal_padding,
        y: area.y + 1, // Add 1 line of breathing room at the top
        width: target_width,
        height: area.height.saturating_sub(2), // Leave room at bottom
    };

    // Focus Mode Styles:
    // 1. Base text is dimmed (DarkGray)
    // 2. Active line is bright (White + Bold)
    // This creates a "fade" effect where only the current thought is in sharp focus.
    app.textarea.set_style(Style::default().fg(Color::DarkGray));
    app.textarea.set_cursor_line_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    
    // Minimalist: No block borders
    app.textarea.set_block(Block::default()); 
    
    // Render the text area in the centered column
    f.render_widget(&app.textarea, text_area);

    // Timer Overlay (Keep at absolute Bottom Right of screen)
    let time_str = format!(
        "{:02}:{:02}",
        app.flow_remaining.as_secs() / 60,
        app.flow_remaining.as_secs() % 60
    );
    
    let timer_width = 10;
    let timer_rect = Rect::new(
        area.width.saturating_sub(timer_width + 2), 
        area.height.saturating_sub(2), 
        timer_width, 
        1
    );
    
    let timer = Paragraph::new(time_str)
        .style(Style::default().fg(if app.flow_remaining.as_secs() < 60 { Color::Red } else { Color::Green }));
    f.render_widget(timer, timer_rect);
}

fn render_history(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app.history.iter().map(|entry| {
        let preview = entry.text.lines().next().unwrap_or("Empty").chars().take(50).collect::<String>();
        let content = format!(
            "{} | {}m | {}", 
            entry.timestamp.format("%Y-%m-%d %H:%M"),
            entry.duration_minutes,
            preview
        );
        ListItem::new(content)
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Flow History "))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, area, &mut app.history_state);
}

fn render_drafts(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app.drafts.iter().map(|d| {
        ListItem::new(Line::from(d.clone()))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Drafts (Enter to open, Del to delete) "))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, area, &mut app.drafts_state);
}

fn render_settings(f: &mut Frame, app: &mut App, area: Rect) {
    // Basic settings display
    let _extension_label = if app.settings.default_extension == "txt" { "(txt)" } else { "(md)" };
    
    let output = vec![
        Line::from(vec![Span::raw(" Settings ").bold()]),
        Line::from(""),
        Line::from(vec![
            Span::raw(" [e] Default Extension: "),
            Span::raw(app.settings.default_extension.clone()).bold().fg(Color::Yellow),
        ]),
        Line::from(vec![
            Span::raw(" Storage Path: "),
            Span::raw(app.settings.storage_path.clone()).italic().fg(Color::Cyan),
        ]),
        Line::from("(Edit storage path in settings.json)"),
        Line::from(""),
        Line::from(" [Esc] Back to Menu"),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Settings ");
    let p = Paragraph::new(output).block(block);
    f.render_widget(p, area);
}

fn parse_markdown_to_lines(input: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut current_spans = Vec::new();
    let mut style = Style::default();

    // Enable basic features
    let parser = Parser::new(input);
    
    for event in parser {
        match event {
            Event::Text(t) => current_spans.push(Span::styled(t.to_string(), style)),
            Event::Code(c) => current_spans.push(Span::styled(c.to_string(), style.bg(Color::DarkGray).fg(Color::White))),
            Event::Start(Tag::Emphasis) => style = style.add_modifier(Modifier::ITALIC),
            Event::End(TagEnd::Emphasis) => style = style.remove_modifier(Modifier::ITALIC),
            Event::Start(Tag::Strong) => style = style.add_modifier(Modifier::BOLD),
            Event::End(TagEnd::Strong) => style = style.remove_modifier(Modifier::BOLD),
            Event::Start(Tag::Heading { .. }) => {
                style = style.add_modifier(Modifier::BOLD).fg(Color::Yellow);
            }
            Event::End(TagEnd::Heading(_)) => {
                style = Style::default();
                lines.push(Line::from(current_spans.clone()));
                current_spans.clear();
                lines.push(Line::from("")); // Space after header
            }
            Event::Start(Tag::Paragraph) => {
                if !current_spans.is_empty() {
                    lines.push(Line::from(current_spans.clone()));
                    current_spans.clear();
                }
            }
            Event::End(TagEnd::Paragraph) => {
                 lines.push(Line::from(current_spans.clone()));
                 current_spans.clear();
                 lines.push(Line::from("")); 
            }
            Event::SoftBreak => {
                current_spans.push(Span::raw(" "));
            }
            Event::HardBreak => {
                lines.push(Line::from(current_spans.clone()));
                current_spans.clear();
            }
            Event::Start(Tag::List(_)) | Event::End(TagEnd::List(_)) => {}
            Event::Start(Tag::Item) => {
                current_spans.push(Span::raw("• "));
            }
            Event::End(TagEnd::Item) => {
                 lines.push(Line::from(current_spans.clone()));
                 current_spans.clear();
            }
            _ => {}
        }
    }
    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }
    lines
}
