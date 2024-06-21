use rspotify::prelude::*;
use rspotify::{scopes, AuthCodePkceSpotify, Config, Credentials, OAuth};

pub type Client = AuthCodePkceSpotify;
use std::path::PathBuf;

fn cache_path() -> PathBuf {
    let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    p.push("ratatui-spotify");
    std::fs::create_dir_all(&p).ok();
    p.push("token.json");
    p
}

pub fn login() -> Client {
    let creds = Credentials::from_env().expect("set RSPOTIFY_CLIENT_ID");
    let oauth = OAuth::from_env(scopes!(
        "user-read-playback-state",
        "user-modify-playback-state",
        "playlist-read-private"
    ))
    .expect("set RSPOTIFY_REDIRECT_URI");
    let config = Config {
        token_cached: true,
        cache_path: cache_path(),
        ..Default::default()
    };
    let mut spotify = AuthCodePkceSpotify::with_config(creds, oauth, config);
    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).expect("login failed");
    spotify
}

pub fn playlists(c: &Client) -> Vec<String> {
    c.current_user_playlists_manual(Some(50), None)
        .map(|p| p.items.into_iter().map(|i| i.name).collect())
        .unwrap_or_default()
}

pub fn tracks(c: &Client, playlist_idx: usize) -> Vec<String> {
    let lists = match c.current_user_playlists_manual(Some(50), None) {
        Ok(l) => l.items,
        Err(_) => return vec![],
    };
    let id = match lists.get(playlist_idx) {
        Some(p) => p.id.clone(),
        None => return vec![],
    };
    c.playlist_items_manual(id, None, None, Some(100), None)
        .map(|p| {
            p.items
                .into_iter()
                .filter_map(|i| match i.track {
                    Some(rspotify::model::PlayableItem::Track(t)) => Some(t.name),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn play(c: &Client, playlist_idx: usize, track_idx: usize) {
    if let Ok(lists) = c.current_user_playlists_manual(Some(50), None) {
        if let Some(p) = lists.items.get(playlist_idx) {
            let _ = c.start_context_playback(
                rspotify::model::PlayContextId::Playlist(p.id.clone()),
                None,
                Some(rspotify::model::Offset::Position(chrono::Duration::seconds(track_idx as i64))),
                None,
            );
        }
    }
}

pub fn toggle_pause(c: &Client) {
    match c.current_playback(None, None::<&[_]>) {
        Ok(Some(pb)) if pb.is_playing => { let _ = c.pause_playback(None); }
        _ => { let _ = c.resume_playback(None, None); }
    }
}

pub fn search(c: &Client, query: &str) -> Vec<String> {
    c.search(query, rspotify::model::SearchType::Track, None, None, Some(30), None)
        .map(|r| match r {
            rspotify::model::SearchResult::Tracks(p) => p.items.into_iter().map(|t| t.name).collect(),
            _ => vec![],
        })
        .unwrap_or_default()
}

pub fn change_volume(c: &Client, delta: i8) {
    if let Ok(Some(pb)) = c.current_playback(None, None::<&[_]>) {
        let cur = pb.device.volume_percent.unwrap_or(50) as i16;
        let _ = c.volume((cur + delta as i16).clamp(0, 100) as u8, None);
    }
}
