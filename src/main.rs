use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use tui::backend::CrosstermBackend;
use tui::Terminal;
mod app;
mod ui;
mod spotify;
mod poller;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let mut app = app::App::new();
    let client = spotify::login();
    app.playlists = spotify::playlists(&client);
    app.client = Some(client);
    let rx = poller::start(app.client.clone().unwrap());

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;
        while let Ok(state) = rx.try_recv() {
            app.playback = state;
        }
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Tab => app.toggle_focus(),
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Enter => app.play_selected(),
                    KeyCode::Char(' ') => app.toggle_pause(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
