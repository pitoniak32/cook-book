## Running
1. `docker compose up -d` (start otel collector, and jaeger)
2. `cargo run` (start axum app)
3. `cargo run --example quick-dev` (run some test api calls)

See traces at `http://localhost:16686/search`

### Credits
Heavily Inspired by Jermey Chone (https://github.com/jeremychone-channel/rust-web-app-preview).
I cannot recommend [Rust Production Coding Playlist](https://youtube.com/playlist?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7&si=2R5_p8LJ64cCmwJ7~) enough.
