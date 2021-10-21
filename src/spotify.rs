use rspotify::prelude::*;
use rspotify::{scopes, AuthCodePkceSpotify, Config, Credentials, OAuth};

pub type Client = AuthCodePkceSpotify;

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
