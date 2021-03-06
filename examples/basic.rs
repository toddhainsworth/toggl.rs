extern crate toggl;

use toggl::entities::{Client, User};
use toggl::session::Session;

fn main() {
    let session = Session::new("token goes here");
    let me = User::me(&session);

    println!("{:?}", me);

    let clients = Client::all(&session);
    println!("{:?}", clients);
}
