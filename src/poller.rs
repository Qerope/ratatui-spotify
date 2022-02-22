use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

use rspotify::prelude::*;

use crate::app::Playback;
use crate::spotify::Client;

pub fn start(client: Client) -> Receiver<Playback> {
    let (tx, rx) = channel();
    thread::spawn(move || loop {
        if let Ok(Some(pb)) = client.current_playback(None, None::<&[_]>) {
            let (title, duration) = match pb.item {
                Some(rspotify::model::PlayableItem::Track(t)) => (t.name, t.duration.num_milliseconds() as u32),
                _ => (String::new(), 0),
            };
            let state = Playback {
                title,
                progress_ms: pb.progress.map(|d| d.num_milliseconds() as u32).unwrap_or(0),
                duration_ms: duration,
                playing: pb.is_playing,
            };
            if tx.send(state).is_err() {
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    });
    rx
}
