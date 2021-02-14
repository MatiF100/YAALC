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
use tui::widgets::{Block, Borders, List, ListItem, ListState, Widget};
use tui::Frame;
use tui::Terminal;

const TASKS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];



pub fn create_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    execute!(io::stdout(), EnterAlternateScreen);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    match Terminal::new(backend) {
        Ok(k) => k,
        Err(e) => panic!("Unexpected error happened: {}", e),
    }
}

pub fn leave_terminal() {
    execute!(io::stdout(), LeaveAlternateScreen);
}

#[allow(dead_code)]
pub fn reenter_terminal() {
    execute!(io::stdout(), EnterAlternateScreen);
}

pub fn events_test(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> std::sync::mpsc::Receiver<Event> {
    let (tx, rx) = channel();
    thread::spawn(move || loop {
        tx.send(read().unwrap()).unwrap();
    });
    rx
}

pub fn draw_frame(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
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
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        draw_list(f, chunks[1]);
    }) {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error happened: {}", e),
    };
}

fn draw_list(f: &mut Frame<tui::backend::CrosstermBackend<std::io::Stdout>>, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal).split(area);

    let tasks: Vec<ListItem> = TASKS
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
        .collect();
    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("Anime"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(" >");

        f.render_widget(tasks.clone(), chunks[0]);
}
