use js_sys::Function;
use wasm_bindgen::prelude::Closure;
use yew::prelude::*;
use ratatui::{prelude::*, widgets::*};

use crate::{console_debug, TERMINAL, console_log, terminal::get_window_size};

pub struct PostViewer {
}

#[derive(Debug)]
pub enum PostViewMsg {
    Resized,
}

impl Component for PostViewer {
    type Message = PostViewMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Bind a function to the "on-resize" window event
        let cb = ctx.link().callback(|()| PostViewMsg::Resized);
        let func = move || cb.emit(());
        let func: Function = Closure::<dyn 'static + Fn()>::new(func).into_js_value().into();
        let window = web_sys::window().unwrap();
        window.set_onresize(Some(&func));
        // Create the viewer
        Self { }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        console_log(format!("Got new message: {msg:?}"));
        match msg {
            PostViewMsg::Resized => {
                TERMINAL.term().backend_mut().resize_buffer();
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut term = TERMINAL.term();
        console_debug(term.size().unwrap());
        term.draw(draw_screen).unwrap();
        term.backend().view()
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
