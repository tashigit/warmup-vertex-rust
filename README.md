# warmup-vertex-rust

Starter template for the **Tashi Vertex Swarm Challenge 2026 — The Stateful Handshake** warm-up.

Clone this repo, get two nodes talking, then build the full challenge on top.

---

## Pre-requisites

- Rust toolchain (`rustup`, `cargo`)
- CMake ≥ 4.0 (required by the `tashi-vertex` build script)

---

## Quick Start

```sh
# 1. Generate a keypair for each node (run twice)
cargo run -- gen-key

# 2. Terminal 1 – Node A
cargo run -- run \
  --bind 127.0.0.1:9000 \
  --secret <SECRET_A> \
  --peer-addr 127.0.0.1:9001 \
  --peer-pubkey <PUBKEY_B>

# 3. Terminal 2 – Node B
cargo run -- run \
  --bind 127.0.0.1:9001 \
  --secret <SECRET_B> \
  --peer-addr 127.0.0.1:9000 \
  --peer-pubkey <PUBKEY_A>
```

You should see both nodes printing `[EVENT]` lines as they exchange raw transactions through Vertex consensus.

---

## Your Challenge

Extend `src/main.rs` to complete the warm-up:

- [ ] **Handshake** — send a signed `HELLO` transaction on startup
- [ ] **Heartbeats** — send a transaction periodically to keep the connection alive
- [ ] **Replicated state** — maintain a local JSON state `{ peer_id, last_seen_ms, role, status }`
- [ ] **Trigger action** — have Agent A change its `role`; Agent B must mirror it in <1 second
- [ ] **Stale detection** — mark a peer as `"stale"` if its heartbeat hasn't been seen in >10 s
- [ ] **Recovery** — show the connection auto-resuming after the stale peer comes back

Look for `// TODO` comments in `src/main.rs` to know where to add your logic.

---

## Submitting

Record a short terminal session (30–60 s) showing:
1. Discovery + handshake
2. Active heartbeats
3. State replication (role change mirrored)
4. A killed-then-recovered peer

Drop it in Discord `#shipping-log` to claim your **Stateful Handshake** badge!

---

*Docs: [Vertex Rust SDK](https://docs.tashi.network/developers/developers/vertex-rs)*
