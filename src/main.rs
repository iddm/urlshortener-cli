const USE_CLIPBOARD_ARGUMENT_NAME: &str = "--use-clipboard";

fn main() {
    use std::env;
    use urlshortener::client::UrlShortener;

    let use_clipboard = env::args()
        .skip(1)
        .any(|arg| arg == USE_CLIPBOARD_ARGUMENT_NAME);

    let mut shortened_links = Vec::new();

    let us = UrlShortener::new().unwrap();
    for link in env::args().skip(1) {
        if link == USE_CLIPBOARD_ARGUMENT_NAME {
            // Ignore the argument if it is not a link.
            continue;
        }

        let mut short_link = Err(urlshortener::providers::ProviderError::Connection);
        for provider in urlshortener::providers::PROVIDERS {
            let res = us.generate(&link, provider);
            if res.is_ok() {
                short_link = res;
                break;
            }
        }
        match short_link {
            Ok(short) => {
                if use_clipboard {
                    shortened_links.push(short.clone());
                }

                println!("{} -> {}", link, short)
            }
            Err(e) => println!("{} -> {:?}", link, e),
        }
    }

    if use_clipboard {
        match cli_clipboard::set_contents(shortened_links.join("\n")) {
            Ok(_) => println!("Shortened links copied to clipboard."),
            Err(e) => eprintln!("Failed to copy shortened links to clipboard: {:?}", e),
        }
    }
}
