# arena-rs

A top-down arena shooter built in Rust + Bevy 0.16. Ships waves of enemies at you with escalating speed. Shoot them. Don't die.

Built as a learnmaxx project — after writing [tiny-autograd-rs](https://github.com/AKMessi/tiny-autograd-rs) and [RustAttention](https://github.com/AKMessi/RustAttention), this was the next step: learn Rust systems design under the pressure of a real-time game loop.

![Rust](https://img.shields.io/badge/Rust-stable-orange?style=flat-square&logo=rust)
![Bevy](https://img.shields.io/badge/Bevy-0.16.1-blue?style=flat-square)

---

## Gameplay

- **WASD** — move
- **Space** — shoot
- **R** — restart after game over

Enemies spawn from all four screen edges and track toward you. Spawn rate and speed increase over time. Kill score persists to disk as a high score between sessions.

---

## Architecture

Five plugins, each owning its own entities and cleanup logic.

```
src/
├── main.rs      — app init, camera shake system
├── player.rs    — movement, input, fire cooldown
├── enemy.rs     — spawner, wave scaling, tracking AI
├── combat.rs    — collision, particles, audio, score, high score persistence
└── ui.rs        — GameState machine, HUD, game over screen
```

**State machine:** `GameState::Playing` → `GameState::GameOver` → back. Every plugin hooks into `OnEnter(GameState::Playing)` for setup and `OnExit` for cleanup. No global reset function — each module cleans up its own entities.

**ECS patterns used:**
- `Single<T>` for guaranteed-unique entity access (player transform in enemy tracking and collision)
- `Without<Enemy>` on player queries to resolve disjoint component access, letting the scheduler run systems in parallel without B0001 panics
- `Commands` deferred despawn — mutations queue after system completion, never mid-iteration
- `Res<T>` frame-driven resource polling for instant HUD updates
- `PlaybackSettings::DESPAWN` for fire-and-forget audio with no entity leaks

**Wave scaling:** `WaveManager` tracks elapsed seconds. Enemy speed scales from 150 to 320 px/s over time. Spawn interval shrinks from 1.0s down to a 0.25s floor.

**Camera shake:** Stress-based model. Hits add 0.4 stress (capped at 1.0). Each frame decays stress by `2.5 * delta_secs`. Shake offset scales as `stress²` for a nonlinear feel — barely perceptible at low stress, violent at high.

**High score:** Written to `highscore.txt` via `std::fs` on game over, loaded at startup via `HighScore::default()`. No dependencies beyond std.

---

## Running

```bash
git clone https://github.com/AKMessi/arena-rs
cd arena-rs
cargo run --release
```

Requires the `assets/` folder at the project root (included in the repo). First compile takes a while — Bevy is a large dependency tree.

---

## Dependencies

```toml
bevy = "0.16.1"
rand = "0.9"
```

---

## What I learned

Coming from Python/ML with no systems programming background, the sharpest edges were:

**The borrow checker in ECS context.** Bevy's B0001 error (conflicting component access) forced understanding of how the scheduler analyzes system signatures. The fix — `Without<Enemy>` on the player query — isn't a workaround, it's telling the scheduler the queries are disjoint so it can run them in parallel safely.

**Deferred mutations.** You can't despawn an entity while iterating a query that contains it. `Commands` buffers despawns until after the system exits, when the borrow is released. Same principle as "don't modify a list while iterating it", just enforced at compile time.

**Modern Fallible Query Management.** The Bevy 0.16 refactor shifted structural assertions. While `Single<T>` provides beautiful fail-fast invariants for unique entities like the player, using the updated `single_mut()` native `Result` unpacker with `if let Ok()` allows fallible systems like the camera shake matrix to handle state transitions gracefully without panicking.

**Plugin architecture pays off immediately.** Splitting into five plugins meant each module reset its own state cleanly on `OnEnter(Playing)` with no cross-module coupling. Adding camera shake to `main.rs` had zero effect on anything else.

---

Part of a Rust learnmaxx series: [tiny-autograd-rs](https://github.com/AKMessi/tiny-autograd-rs) → [RustAttention](https://github.com/AKMessi/RustAttention) → arena-rs
