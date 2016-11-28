extern crate urlshortener;

use urlshortener::UrlShortener;
use std::env;

fn main() {
    let us = UrlShortener::new(); 
    for link in env::args().skip(1) {
        if let Ok(short) = us.try_generate(link.clone()) {
            println!("{} -> {}", link, short);
        } else {
            println!("{} -> Error", link);
        }
    }
}
