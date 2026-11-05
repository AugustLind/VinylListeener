# Vinyl Listener

An early Rust prototype for a Raspberry Pi project that would identify music
playing on a turntable and display the current song.

## Status

The current program creates a Chromaprint fingerprint from one second of
generated silent stereo samples (44,100 Hz) and prints the fingerprint.
It does not yet capture turntable or microphone audio, decode files, query
AcoustID, identify songs, or drive a display.

The `ApiListener` type is currently a fingerprinting experiment; its name does
not mean it makes an API request. Symphonia is declared as a dependency for
future decoding work but is not used by the current program.

## Run the prototype

Install a Rust toolchain supporting the 2024 edition (Rust 1.85 or later).
A standard Rust build toolchain, including a platform linker, is required.
Run from the crate directory:

```bash
git clone https://github.com/AugustLind/VinylListeener.git
cd VinylListeener/vinyl_listener
cargo run
```

The executable prints a greeting and the generated fingerprint. It currently
needs neither a Raspberry Pi, an audio device, nor an API key.

## Configuration

A real API key was previously committed in `vinyl_listener/src/.env`.
It must be treated as exposed and revoked/replaced through the issuing account.
Removing the file from the current branch does not invalidate the old key or
erase earlier commits, downloaded copies, or cached views.

For future AcoustID integration, a placeholder is provided:

```bash
cd vinyl_listener  # from the repository root
cp .env.example .env
```

Only put a newly issued key in the local `.env` file. The current program does
not load environment files or read `ACOUST_ID_API_KEY`; no key is needed to run
the prototype. Add configuration loading when implementing API integration.

Local `.env` and `.env.*` files are ignored at any directory depth; the safe
`.env.example` template remains trackable. Never put a real key in that template.

## Layout

| File | Purpose |
|---|---|
| `vinyl_listener/Cargo.toml` | Rust package and dependencies |
| `vinyl_listener/src/main.rs` | Prototype entry point |
| `vinyl_listener/src/api_listener.rs` | Fingerprint generation experiment |
| `vinyl_listener/.env.example` | Placeholder for future API configuration |

## Next steps

- Capture actual audio or decode an audio file.
- Generate fingerprints from real samples and report errors clearly.
- Implement AcoustID lookup using local configuration.
- Add song metadata and a display interface.
- Document and test the required Raspberry Pi/audio hardware.

## License

[MIT](LICENSE).
