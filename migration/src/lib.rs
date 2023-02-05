pub use migration_macros::*;

pub mod migrations;

#[test]
fn test() {
    println!("PERFORMING MIGRATIONS...");
    migrate_from!("");
}
