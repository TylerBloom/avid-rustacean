use ratatui::{prelude::*, widgets::*};

use crate::{app::CursorMap, console_debug, console_log, terminal::get_window_size};

#[derive(Debug, PartialEq)]
pub struct Home {}

impl Home {
    pub fn create(map: &mut CursorMap) -> Self {
        Self {}
    }

    pub fn draw(&self, chunk: Rect, frame: &mut Frame) -> Rect {
        draw_screen(chunk, frame)
    }
}

fn draw_screen(rect: Rect, frame: &mut Frame) -> Rect {
    console_log("Drawing home page...");
    console_log(format!("Given area: {rect:?}"));
    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat((rect.width as usize) / s.len() + 4);
    long_line.push('\n');

    let area = Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    };
    console_log(format!("Rendering area: {area:?}"));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let digest = Rect {
        x: 0,
        y: chunks.last().unwrap().y + chunks.last().unwrap().height,
        width: rect.width,
        height: 0,
    };
    console_log(format!("Returning area: {digest:?}"));

    let text = vec![
        Line::from("This is a line "),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_blue()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.on_green()),
        Line::from("This is a line".green().italic()),
        Line::from(vec![
            "Masked text: ".into(),
            Span::styled(
                Masked::new("password", '*'),
                Style::default().fg(Color::Red),
            ),
        ]),
    ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    console_log("Rendering home page...");
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), no wrap"));
    frame.render_widget(paragraph, chunks[0]);
    console_log("Rendered first paragraph...");

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), with wrap"))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[1]);
    console_log("Rendered second paragraph...");

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Right alignment, with wrap"))
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[2]);
    console_log("Rendered third paragraph...");

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Center alignment, with wrap, with scroll"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[3]);
    console_log("Rendered fourth paragraph...");

    digest
}
