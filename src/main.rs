use crossterm::event::Event;

mod anilist;
mod terminal;

#[tokio::main]
async fn main() {
    println!("{:#?}", anilist::test().await);

    let mut terminal = terminal::create_terminal();
    terminal::draw_frame(&mut terminal);
    let event = terminal::events_test(&mut terminal);

    loop {
        let ev = event.recv().unwrap();
        match ev {
            Event::Key(key) => match key {
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Char('q'),
                    modifiers: _,
                } => break,
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Char('w'),
                    modifiers: _,
                } => terminal::leave_terminal(),
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Char('e'),
                    modifiers: _,
                } => terminal::reenter_terminal(),
                _ => println!("Pressed: {:?}", key),
            },
            Event::Resize(_, _) => terminal::draw_frame(&mut terminal),
            _ => (),
        }
    }
}
