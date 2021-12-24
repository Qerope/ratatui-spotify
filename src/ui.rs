use crate::app::{App, Focus};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Frame;
use tui::widgets::Gauge;
use ratatui::widgets::Gauge;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(f.size());
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(rows[0]);

    let highlight = Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD);
    let border = |focused: bool| {
        if focused { Style::default().fg(Color::Green) } else { Style::default() }
    };

    let playlists: Vec<ListItem> = app.playlists.iter().map(|p| ListItem::new(p.as_str())).collect();
    let mut pstate = ListState::default();
    pstate.select(Some(app.playlist_sel));
    f.render_stateful_widget(
        List::new(playlists)
            .block(Block::default().title("Playlists").borders(Borders::ALL)
                .border_style(border(app.focus == Focus::Playlists)))
            .highlight_style(highlight),
        cols[0],
        &mut pstate,
    );

    let tracks: Vec<ListItem> = app.tracks.iter().map(|t| ListItem::new(t.as_str())).collect();
    let mut tstate = ListState::default();
    tstate.select(Some(app.track_sel));
    f.render_stateful_widget(
        List::new(tracks)
            .block(Block::default().title("Tracks").borders(Borders::ALL)
                .border_style(border(app.focus == Focus::Tracks)))
            .highlight_style(highlight),
        cols[1],
        &mut tstate,
    );

    let ratio = if app.playback.duration_ms == 0 {
        0.0
    } else {
        (app.playback.progress_ms as f64 / app.playback.duration_ms as f64).min(1.0)
    };
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(app.playback.title.as_str()).borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(ratio),
        rows[1],
    );
}
