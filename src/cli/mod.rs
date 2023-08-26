use clap::{Args, Parser, Subcommand};

use self::{
    console::ConsoleCommand,
    identity::{create::CreateIdentityCommand, verify::VerifyIdentityCommand},
    profile::{
        gossip::GossipProfileCommand, rpc::RpcProfileCommand, storage::StorageProfileCommand,
    },
    reset::{events::EventsResetCommand, full::FullResetCommand},
    start::StartCommand,
    status::StatusCommand,
};

mod console;
mod identity;
mod profile;
mod reset;
mod start;
mod status;

#[derive(Parser, Debug)]
#[command(name = "teleport")]
#[command(
    about = "A fast implementation of a Farcaster Hub, in Rust",
    author = "Haardik (haardik@learnweb3.io)"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start(StartCommand),
    #[command(name = "identity", about = "Create and verify Peer IDs")]
    Identity(IdentityArgs),
    Status(StatusCommand),
    #[command(
        name = "profile",
        about = "Profile the Hub's RPC, Storge, and Gossip Server's performance"
    )]
    Profile(ProfileArgs),
    #[command(name = "reset", about = "Reset parts of the Hub's database")]
    Reset(ResetArgs),
    Console(ConsoleCommand),
}

#[derive(Args, Debug)]
pub struct IdentityArgs {
    #[command(subcommand)]
    command: IdentityCommands,
}

#[derive(Subcommand, Debug)]
pub enum IdentityCommands {
    Create(CreateIdentityCommand),
    Verify(VerifyIdentityCommand),
}

#[derive(Args, Debug)]
pub struct ProfileArgs {
    #[command(subcommand)]
    command: ProfileCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    Gossip(GossipProfileCommand),
    Rpc(RpcProfileCommand),
    Storage(StorageProfileCommand),
}

#[derive(Args, Debug)]
pub struct ResetArgs {
    #[command(subcommand)]
    command: ResetCommands,
}

#[derive(Subcommand, Debug)]
pub enum ResetCommands {
    Events(EventsResetCommand),
    Full(FullResetCommand),
}
