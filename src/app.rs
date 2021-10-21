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
    pub client: Option<crate::spotify::Client>,
}

impl App {
    pub fn new() -> App {
        App {
            focus: Focus::Playlists,
            playlists: vec![],
            tracks: vec![],
            playlist_sel: 0,
            track_sel: 0,
            client: None,
        }
    }
                self.load_tracks();
                self.load_tracks();

    pub fn load_tracks(&mut self) {
        if let Some(c) = &self.client {
            self.tracks = crate::spotify::tracks(c, self.playlist_sel);
            self.track_sel = 0;
        }
    }
}
