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


// Vulnerable Retrieve function - Insuffiecient Input Validation allows for Path Disclosure necessary for Injection and File inculsion vulnerabilities 
// Stop them in the Recon phase
#[get("/<id>")]
async fn retrieve(id: &str) -> Option<File> {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let filename = Path::new(upload_dir).join(id);
    File::open(&filename).await.ok()
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
