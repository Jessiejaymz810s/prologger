/**
 * prologger — A production-grade, ergonomic Rust logging library.
 *
 * This npm package is a metadata wrapper for the prologger Rust crate.
 * It does NOT provide JavaScript functionality — it exists so that
 * prologger is discoverable via the GitHub npm Packages registry.
 *
 * To use prologger in your Rust project:
 *
 *   1. Add to your Cargo.toml:
 *      [dependencies]
 *      prologger = "0.3"
 *      log = "0.4"
 *
 *   2. Initialize in your code:
 *      use log::{info, warn, error};
 *      fn main() {
 *          prologger::init();
 *          info!("Application started");
 *      }
 *
 * For full documentation, visit:
 *   - GitHub:  https://github.com/Jessiejaymz810s/prologger
 *   - Docs:    https://docs.rs/prologger
 *   - Crates:  https://crates.io/crates/prologger
 *
 * @module prologger
 * @author Jessiejaymz810s
 * @license MIT
 */

module.exports = {
  name: "prologger",
  version: "0.3.0",
  language: "rust",
  registry: "https://crates.io/crates/prologger",
  documentation: "https://docs.rs/prologger",
  repository: "https://github.com/Jessiejaymz810s/prologger",
  install: 'Add to Cargo.toml: prologger = "0.3"',
};
