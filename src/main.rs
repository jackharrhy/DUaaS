#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate base64;

use std::fs::File;
use std::io::Read;

fn grab_random_base64(amount: usize) -> String {
    let mut urandom = File::open("/dev/urandom").unwrap();
    let mut buf = vec![0u8; amount];
    urandom.read_exact(&mut buf).unwrap();

    base64::encode(&buf)
}

#[get("/")]
fn return_128_bytes() -> String {
    grab_random_base64(128)
}

#[get("/<amount>")]
fn return_specified_bytes(amount: usize) -> String {
    grab_random_base64(amount)
}

fn main() {
    rocket::ignite().mount("/", routes![return_100_bytes, return_specified_bytes]).launch();
}
