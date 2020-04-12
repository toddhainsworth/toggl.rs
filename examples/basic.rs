extern crate toggl;

use toggl::entities::{Client, User};
use toggl::session::Session;

fn main() {
    let session = Session::new("token goes here");
    let me = User::me(&session);

    println!("{:?}", me);

    let client = Client::get(&session, "id goes here");
    println!("{:?}", client);
}
