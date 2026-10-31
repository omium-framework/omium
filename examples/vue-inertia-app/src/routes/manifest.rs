//! Route manifest for `./omnisan route:list` (keep in sync with `routes/mod.rs`).

use super::api;

/// One line in the route table (`php artisan route:list`).
#[derive(Debug, Clone, Copy)]
pub struct RouteEntry {
    pub methods: &'static str,
    pub uri: &'static str,
    pub action: &'static str,
}

pub const WEB_ENTRIES: &[RouteEntry] = &[
    RouteEntry {
        methods: "GET",
        uri: "/",
        action: "dashboard::show",
    },
    RouteEntry {
        methods: "GET",
        uri: "/examples",
        action: "examples::index",
    },
    RouteEntry {
        methods: "GET|POST",
        uri: "/examples/new",
        action: "examples::create_form|create",
    },
    RouteEntry {
        methods: "GET|POST",
        uri: "/examples/{id}/edit",
        action: "examples::edit_form|update",
    },
    RouteEntry {
        methods: "DELETE",
        uri: "/examples/{id}",
        action: "examples::destroy",
    },
    RouteEntry {
        methods: "GET",
        uri: "/about",
        action: "about::show",
    },
];

/// Prints the route table to stdout (used by the `route-list` binary).
pub fn print_route_table() {
    println!("{:<7}  {:<32}  {}", "Method", "URI", "Action");
    println!("{}", "-".repeat(72));
    for entry in WEB_ENTRIES {
        print_entry(*entry);
    }
    for entry in api::ENTRIES {
        print_entry(*entry);
    }
    println!("{:<7}  {:<32}  {}", "GET", "/build/{path}", "(static Vite assets)");
}

fn print_entry(entry: RouteEntry) {
    println!(
        "{:<7}  {:<32}  {}",
        entry.methods, entry.uri, entry.action
    );
}
