// integration testing

use adows::*;

#[test]
fn create_server() {
    Server::new("assets/www", 8080).unwrap();
}
