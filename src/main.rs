#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use std::env;

use rocket::State;
use rocket_contrib::templates::Template;

#[derive(Debug, Serialize)]
struct KeyValuePair {
    key: String,
    value: String,
}

impl KeyValuePair {
    pub fn from(text: &String) -> KeyValuePair {
        let parts: Vec<&str> = text.split(':').collect();
        KeyValuePair {
            key: parts[0].to_string(),
            value: if parts.len() >= 2 {
                parts[1].to_string()
            } else {
                String::from("NO VALUE")
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct WebContext {
    title: String,
    items: Vec<KeyValuePair>,
}

impl WebContext {
    pub fn from(input: &Vec<String>) -> WebContext {
        WebContext {
            title: String::from("Table"),
            items: input.into_iter().map(KeyValuePair::from).collect(),
        }
    }
}

#[get("/")]
fn show(context: State<WebContext>) -> Template {
    Template::render("key_value_table", context.inner())
}

fn main() {
    // skip(1) so that the first arg (which is the path to executable)
    // is not included.
    let kvs: Vec<String> = env::args().skip(1).collect();
    rocket::ignite()
        .manage(WebContext::from(&kvs))
        .mount("/", routes![show])
        .attach(Template::fairing())
        .launch();
}
