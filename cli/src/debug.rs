use cloudstate_runtime::print::print_database;
use redb::Database;

#[test]
fn test_print() {
    tracing_subscriber::fmt::init();
    print_database(&Database::open("./cloudstate").unwrap());
}
