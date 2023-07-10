mod bomanis_bank;

fn main() {
    let mut session = bomanis_bank::Session::new();
    session.start();
}
