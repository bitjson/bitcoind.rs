use node::db::test_ldb;
use node::peer::connect;

pub fn start() {
    test_ldb();

    connect("127.0.0.1", 8333);
}
