//! Tashi Vertex Warmup — The Stateful Handshake
//! Starter skeleton for participants.
//!
//! This gets two Vertex nodes connected and printing received transactions.
//! Your job is to build the warm-up challenge on top of this foundation.
//!
//! Usage:
//!   # Step 1: generate a keypair for EACH node (run twice)
//!   cargo run -- gen-key
//!
//!   # Step 2: start each node in its own terminal
//!   cargo run -- run \
//!     --bind 127.0.0.1:9000 \
//!     --secret <SECRET_A> \
//!     --peer-addr 127.0.0.1:9001 \
//!     --peer-pubkey <PUBKEY_B>
//!
//!   # (Terminal 2)
//!   cargo run -- run \
//!     --bind 127.0.0.1:9001 \
//!     --secret <SECRET_B> \
//!     --peer-addr 127.0.0.1:9000 \
//!     --peer-pubkey <PUBKEY_A>

use clap::{Parser, Subcommand};
use tashi_vertex::{Context, Engine, KeyPublic, KeySecret, Message, Options, Peers, Socket, Transaction};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(name = "node", about = "Tashi Vertex Warmup skeleton")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a new Ed25519 keypair
    GenKey,
    /// Run a node and connect to a peer
    Run {
        #[arg(long)]
        bind: String,
        #[arg(long)]
        secret: String,
        #[arg(long)]
        peer_addr: String,
        #[arg(long)]
        peer_pubkey: String,
    },
}

// ---------------------------------------------------------------------------
// TODO: Define your shared state struct here.
//
// The warm-up requires each node to maintain a replicated state like:
//   {
//     peer_id:      String,
//     last_seen_ms: u64,
//     role:         String,   // e.g. "scout" | "carrier"
//     status:       String,   // e.g. "ready" | "busy" | "stale"
//   }
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // ── Key generation ───────────────────────────────────────────────────
        Command::GenKey => {
            let secret = KeySecret::generate();
            println!("Secret (keep private): {secret}");
            println!("Public (share freely): {}", secret.public());
        }

        // ── Node runtime ─────────────────────────────────────────────────────
        Command::Run { bind, secret, peer_addr, peer_pubkey } => {
            let key: KeySecret = secret.parse()?;
            let peer_pub: KeyPublic = peer_pubkey.parse()?;

            // Build the peer registry — every participant must be listed here
            let mut peers = Peers::new()?;
            peers.insert(&peer_addr, &peer_pub, Default::default())?;
            peers.insert(&bind, &key.public(), Default::default())?;

            // Start the consensus engine
            let context = Context::new()?;
            let socket = Socket::bind(&context, &bind).await?;
            let engine = Engine::start(&context, socket, Options::default(), &key, peers)?;

            println!("Node started. Bound to {bind}");
            println!("My public key: {}", key.public());
            println!("Waiting for consensus events...\n");

            // TODO: spawn a background task to send periodic HEARTBEAT transactions

            // Receive loop — every Message::Event here has been consensus-ordered
            while let Some(msg) = engine.recv_message().await? {
                match msg {
                    Message::Event(event) => {
                        for tx in event.transactions() {
                            // TODO: parse the transaction payload (e.g. JSON)
                            // TODO: update your local shared state
                            // TODO: detect stale peers (check last_seen_ms)
                            println!("[EVENT] raw tx: {:?}", tx);
                        }
                    }
                    Message::SyncPoint(_) => {
                        println!("[SYNC POINT]");
                    }
                }
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Helper: allocate and send a raw byte slice as a transaction
// ---------------------------------------------------------------------------
#[allow(dead_code)]
fn send_bytes(engine: &Engine, data: &[u8]) -> tashi_vertex::Result<()> {
    let mut tx = Transaction::allocate(data.len());
    tx.copy_from_slice(data);
    engine.send_transaction(tx)
}
