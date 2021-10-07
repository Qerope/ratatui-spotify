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
    let mut spotify = AuthCodePkceSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).expect("login failed");
    spotify
}
