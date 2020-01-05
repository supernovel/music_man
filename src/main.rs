#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")] 
fn index() -> &'static str {
     "Hello, world!" 
}

#[get("/add_music")]
fn add_music() -> &'static str {
    "ADD_MUSIC_LOGIC"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, add_music])
        .launch(); 
}
