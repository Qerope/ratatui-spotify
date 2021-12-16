#[derive(PartialEq, Clone, Copy)]
pub enum Focus {
    Playlists,
    Tracks,
}

#[derive(Default, Clone)]
pub struct Playback {
    pub title: String,
    pub progress_ms: u32,
    pub duration_ms: u32,
    pub playing: bool,
}

pub struct App {
    pub focus: Focus,
    pub playlists: Vec<String>,
    pub tracks: Vec<String>,
    pub playlist_sel: usize,
    pub track_sel: usize,
    pub client: Option<crate::spotify::Client>,
    pub playback: Playback,
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
            playback: Playback::default(),
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Playlists => Focus::Tracks,
            Focus::Tracks => Focus::Playlists,
        };
    }

    pub fn next(&mut self) {
        match self.focus {
            Focus::Playlists if !self.playlists.is_empty() => {
                self.playlist_sel = (self.playlist_sel + 1) % self.playlists.len();
                self.load_tracks();
            }
            Focus::Tracks if !self.tracks.is_empty() => {
                self.track_sel = (self.track_sel + 1) % self.tracks.len();
            }
            _ => {}
        }
    }

    pub fn previous(&mut self) {
        match self.focus {
            Focus::Playlists if !self.playlists.is_empty() => {
                self.playlist_sel = (self.playlist_sel + self.playlists.len() - 1) % self.playlists.len();
                self.load_tracks();
            }
            Focus::Tracks if !self.tracks.is_empty() => {
                self.track_sel = (self.track_sel + self.tracks.len() - 1) % self.tracks.len();
            }
            _ => {}
        }
    }

    pub fn load_tracks(&mut self) {
        if let Some(c) = &self.client {
            self.tracks = crate::spotify::tracks(c, self.playlist_sel);
            self.track_sel = 0;
        }
    }

    pub fn play_selected(&mut self) {
        if let Some(c) = &self.client {
            crate::spotify::play(c, self.playlist_sel, self.track_sel);
        }
    }

    pub fn toggle_pause(&mut self) {
        if let Some(c) = &self.client {
            crate::spotify::toggle_pause(c);
        }
    }
}
