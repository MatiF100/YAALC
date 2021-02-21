mod anilist;
mod app;
mod terminal;

#[tokio::main]
async fn main() {
    //println!("{:#?}", anilist::test().await.get("data").unwrap().get("Page").unwrap());

    //dbg!(&test);

    let mut app = app::App::new("Lista anime".to_owned());
    app.authorize();

    let test: app::RecievedData = serde_json::from_value(anilist::test(&app).await).unwrap();

    let dummy_list = app::StatefulList::with_items(test.data.unwrap().page.unwrap().media);
    app.animes = dummy_list;

    let mut terminal = terminal::create_terminal();
    terminal::draw_frame(&mut terminal, &mut app);
    let event = terminal::events_test();

    while !app.should_exit {
        let ev = event.recv().unwrap();
        app.handle_input(ev).await;
        terminal::draw_frame(&mut terminal, &mut app);
    }
    terminal::leave_terminal();
}
