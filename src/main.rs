#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate base64;
extern crate ecoji;

use std::fs::File;
use std::io::Read;
use std::str;

fn grab_random_bytes(amount: usize) -> Vec<u8> {
    let mut urandom = File::open("/dev/urandom").unwrap();
    let mut buf = vec![0u8; amount];
    urandom.read_exact(&mut buf).unwrap();
    buf
}

fn grab_random_bytes_as_string(amount: usize) -> String {
    grab_random_bytes(amount)
        .iter()
        .map(|item| format!("{:b}", item))
        .collect::<String>()
}

fn grab_random_base64(amount: usize) -> String {
    let buf = grab_random_bytes(amount);
    base64::encode(&buf)
}

fn grab_random_ecoji(amount: usize) -> String {
    let b64_string = grab_random_base64(amount);

    let mut output: Vec<u8> = Vec::new();

    ecoji::encode(&mut b64_string.as_bytes(), &mut output).expect("Invalid Base64 :(");

    str::from_utf8(&output).unwrap().to_string()
}

// Binary
#[get("/binary")]
fn return_128_bytes() -> String {
    grab_random_bytes_as_string(128)
}

#[get("/binary/<amount>")]
fn return_specified_bytes(amount: usize) -> String {
    grab_random_bytes_as_string(amount)
}

// Base64
#[get("/base64")]
fn return_128_bytes_as_base64() -> String {
    grab_random_base64(128)
}

#[get("/base64/<amount>")]
fn return_specified_bytes_as_base_64(amount: usize) -> String {
    grab_random_base64(amount)
}

// Ecoji
#[get("/ecoji")]
fn return_128_bytes_as_ecoji() -> String {
    grab_random_ecoji(128)
}

#[get("/ecoji/<amount>")]
fn return_specified_bytes_as_ecoji(amount: usize) -> String {
    grab_random_ecoji(amount)
}

#[get("/")]
fn index() -> &'static str {
    "DUaaS -> Dev Urandom as a Service

  USAGE

    GET /binary

      read and return 128 lines from /dev/urandom as binary
      formatted as a string

    GET /binary/<usize>

      read and return <usize> number of lines from
      /dev/urandom as binary formatted as a string

    GET /base64

      read and return 128 lines from /dev/urandom as base64

    GET /base64/<usize>

      read and return <usize> number of lines from
      /dev/urandom as base64

    GET /ecoji

      read and return 128 lines from /dev/urandom as base64
      interpreted as emoji using ecoji

    GET /ecoji/<usize>

      read and return <usize> number of lines from
      /dev/urandom as base64 interpreted as emoji using ecoji"
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                return_128_bytes,
                return_specified_bytes,
                return_128_bytes_as_base64,
                return_specified_bytes_as_base_64,
                return_128_bytes_as_ecoji,
                return_specified_bytes_as_ecoji,
            ],
        )
        .launch();
}
