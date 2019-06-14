fn main() {
    use std::env;
    use urlshortener::client::UrlShortener;

    let us = UrlShortener::new().unwrap();
    for link in env::args().skip(1) {
        let mut short_link = Err(urlshortener::providers::ProviderError::Connection);
        for provider in urlshortener::providers::PROVIDERS {
            let res = us.generate(&link, provider);
            if res.is_ok() {
                short_link = res;
                break;
            }
        }
        match short_link {
            Ok(short) => println!("{} -> {}", link, short),
            Err(e) => println!("{} -> {:?}", link, e),
        }
    }
}
