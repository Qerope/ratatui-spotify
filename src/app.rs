#[derive(PartialEq, Clone, Copy)]
pub enum Focus {
    Playlists,
    Tracks,
}

pub struct App {
    pub focus: Focus,
    pub playlists: Vec<String>,
    pub tracks: Vec<String>,
    pub playlist_sel: usize,
    pub track_sel: usize,
}

impl App {
    pub fn new() -> App {
        App {
            focus: Focus::Playlists,
            playlists: vec!["Liked Songs".into(), "Discover Weekly".into(), "Road trip".into()],
            tracks: vec![],
            playlist_sel: 0,
            track_sel: 0,
        }
    }
}
