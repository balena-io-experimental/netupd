mod opts;

use std::net::ToSocketAddrs;

use anyhow::{Context, Result};

use crate::opts::Opts;

use clap::Parser;

use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    let app = Router::new().route("/", get(root));

    let mut resolved: Vec<_> = opts
        .address
        .to_socket_addrs()
        .with_context(|| format!("Failed to resolve '{}'", opts.address))?
        .collect();

    let address = resolved.pop().context("No addresses resolved")?;

    axum::Server::try_bind(&address)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!\n"
}
