#[macro_use] extern crate rocket;

mod paste_id;

use paste_id::PasteId;
use std::path::Path;
use rocket::tokio::fs::File;
use rocket::request::FromParam;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");



// Solution - validate the ID before you use it 
// ID validation checks if not return an error (paste_id.rs) 
#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}


#[get("/")]
fn index() -> &'static str {
    "
    WELCOME TO YOUR SECURITY JOURNEY 

      Now that you are POSTed up, GET out on the PATH and see if you can find some RUSTY passwords
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve])
}
