mod bomanis_bank;

fn main() {
    let session = bomanis_bank::Session::new();
    session.start();
}
