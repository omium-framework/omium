//! `rustbridge route:list` command.

use rustbridge_http::default_route_manifest;

/// Prints the default framework route manifest.
pub fn route_list() {
    println!("Method  URI");
    for (method, path) in default_route_manifest() {
        println!("{method:<6}  {path}");
    }
    println!();
    println!("Application routes (e.g. sample-app): GET /");
}
