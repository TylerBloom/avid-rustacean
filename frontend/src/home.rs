use ratatui::Frame;

use crate::app::CursorMap;


#[derive(Debug, PartialEq)]
pub struct Home {
}

impl Home {
    pub fn create(map: &mut CursorMap) -> Self {
        todo!()
    }

    pub fn draw(&self, frame: &mut Frame) {
        todo!()
    }
}

fn draw_screen(frame: &mut Frame) {
    console_log("Drawing frame...");
    let (width, height) = get_window_size();
    let mut rect = frame.size();
    console_debug(rect);
    rect.height = height;
    console_debug(rect);

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat((width as usize) / s.len() + 4);
    long_line.push('\n');

    console_log("Rendering block...");
    let block = Block::default().black();
    frame.render_widget(block, rect);

    console_log("Creating layout...");
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(rect);
    console_log("Created layout!!");

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

    console_log("Drawing first paragraph...");
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), no wrap"));
    frame.render_widget(paragraph, chunks[0]);

    console_log("Drawing second paragraph...");
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), with wrap"))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[1]);

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Right alignment, with wrap"))
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[2]);

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Center alignment, with wrap, with scroll"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[3]);
    console_log("Finished drawing frame...");
}
