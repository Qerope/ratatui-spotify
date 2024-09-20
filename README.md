# ratatui-spotify

A terminal client for Spotify (needs Premium for playback control).

    export RSPOTIFY_CLIENT_ID=...
    export RSPOTIFY_REDIRECT_URI=http://localhost:8888/callback
    cargo run --release

Keys: `Tab` switch pane, `j`/`k` move, `Enter` play, `Space` pause, `/`
search, `+`/`-` volume, `q` quit.

The token is cached in `~/.config/ratatui-spotify/token.json`. Playback goes
to the currently active device; start Spotify somewhere first.
