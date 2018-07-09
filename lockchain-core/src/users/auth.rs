//! Provides an authentication module backed by PAM
//!
//! The way a user is authenticated is via the `lockchain` group
//! and a simple writing/ deleting of a lock file.

// use nix::sys::wait::*;
// use nix::unistd::{fork, ForkResult};

use pam_auth::Authenticator;
use errors::AuthError;

/// Simple way to authenticate a user for administrative actions
///
/// Attempts to open a PAM session for the provided user/pw combination
/// then attempts to write to a tmpfile in the lockchain config directory.
/// If this action is successful the user is either the same running the
/// lockchain server *or* has access to the file via group permissions.
///
/// This does rely on `lockchain` being properly configured on the server
/// i.e. not using public permissions for the configuration/ state directory.
///
/// **Note** as of `lockchain v0.9.0` this function has not been implemented
/// yet due to issues in the `pam-auth` dependency.
#[allow(unused_variables)]
pub fn pam_authenticate(username: &str, password: &str) -> Result<(), AuthError> {
    // Err(AuthError::FailedPAM)

    // match fork().map_err(|_| AuthError::FailedFork)? {
    //     ForkResult::Parent { child } => {
    //         waitpid(child, None).unwrap();
    //         // kill(child, SIGKILL).expect("kill failed");
    //     }
    //     ForkResult::Child => {
    // let mut auth = Authenticator::new("lockchain").ok_or(AuthError::FailedPAM)?;

    use std::error::Error;
    let service = "lockchain-core";

    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Service:  {}", service);

    let mut auth = Authenticator::new(service).unwrap();
    auth.set_credentials(username, password);

    match auth.authenticate() {
        Ok(()) => println!("authenticate() OK!"),
        Err(e) => {
            println!("authenticate() FAILED!");
            println!("{}", e.description());
            println!("{:#?}", e.cause());
        }
    }

    match auth.open_session() {
        Ok(()) => println!("open_session() OK!"),
        Err(e) => {
            println!("open_session() FAILED!");
            println!("{}", e.description());
            println!("{:#?}", e.cause());
        }
    }

    Ok(())

    // auth.set_credentials(username, password);
    // auth.authenticate().map_err(|_| AuthError::InvalidUser)?;
    // auth.open_session().map_err(|_| AuthError::FailedPAM)?;

    // use std::process::Command;
    // let output = Command::new("su")
    //     .arg(username)
    //     .output()
    //     .expect("failed to execute process");
    // println!("whoami: {:#?}", String::from_utf8(output.stdout).unwrap());

    // ::std::process::exit(255);
    //     }
    // }

    // Ok(())
}
