//! Core lockchain application server

extern crate clap;

extern crate lockchain_core as core;
extern crate lockchain_files as files;
extern crate lockchain_http as http;

use crate::core::EncryptedBody;
use crate::files::FileVault;
use crate::http::{create_server, state::ApiState};

fn main() {
    let state = ApiState::<EncryptedBody, FileVault<EncryptedBody>> {
        bound_scope: true,
        working_dir: ".".into(),

        // This is a dangerous option
        administrative: true,
        ..Default::default()
    };

    let server = create_server("localhost", "9999", state);
    server.unwrap().run();
}
