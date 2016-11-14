extern crate urlshortener;

use urlshortener::UrlShortener;
use std::env;

fn main() {
    let us = UrlShortener::new(); 
    for (i, link) in env::args().skip(1).enumerate() {
        if let Ok(short) = us.try_generate(link) {
            println!("{}: {}", i + 1, short);
        } else {
            println!("{}: Error", i);
        }
    }
}
