use crossterm::{
    event::{read, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::sync::mpsc::channel;
use std::thread;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget, Wrap};
use tui::Frame;
use tui::Terminal;

use crate::app::{App, AppMode, StatefulList};

pub fn create_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    execute!(io::stdout(), EnterAlternateScreen);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    match Terminal::new(backend) {
        Ok(k) => k,
        Err(e) => panic!("Unexpected error happened: {}", e),
    }
}

pub fn process_event(event: Event) {}

pub fn leave_terminal() {
    execute!(io::stdout(), LeaveAlternateScreen);
}

#[allow(dead_code)]
pub fn reenter_terminal() {
    execute!(io::stdout(), EnterAlternateScreen);
}

pub fn events_test() -> std::sync::mpsc::Receiver<Event> {
    let (tx, rx) = channel();

    thread::spawn(move || loop {
        tx.send(read().unwrap()).unwrap();
    });
    rx
}

pub fn draw_frame(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) {
    match terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Search").borders(Borders::ALL);
        let text = match app.mode {
            AppMode::INPUT => format!("{}_", app.search_bar),
            AppMode::NORMAL => app.search_bar.clone(),
        };
        let paragraph = Paragraph::new(text)
            .block(block.clone())
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);
        draw_list(f, chunks[1], app);
        f.render_widget(block.clone(), chunks[2]);
    }) {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error happened: {}", e),
    };
}

fn draw_list(
    f: &mut Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
    area: Rect,
    app: &mut App,
) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let tasks: Vec<ListItem> = app
        .animes
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.title.get_title()))]))
        .collect();
    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("Anime"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(" >");

    //f.render_widget(tasks.clone(), chunks[0]);
    f.render_stateful_widget(tasks, chunks[0], &mut app.animes.state);
    //println!("{:?}", app.animes.state);
}
