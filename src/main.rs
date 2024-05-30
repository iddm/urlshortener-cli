const GET_FROM_CLIPBOARD_ARGUMENT_NAME: &str = "--get-from-clipboard";
const SET_TO_CLIPBOARD_ARGUMENT_NAME: &str = "--set-to-clipboard";
const DISABLE_NOTIFICATIONS: &str = "--disable-notifications";

fn shorten_one<S: AsRef<str>>(url: S) -> Result<String, urlshortener::providers::ProviderError> {
    use urlshortener::client::UrlShortener;

    let us = UrlShortener::new().unwrap();
    for provider in urlshortener::providers::PROVIDERS {
        let res = us.generate(url.as_ref(), provider);
        if res.is_ok() {
            return res;
        }
    }

    Err(urlshortener::providers::ProviderError::Connection)
}

fn notify(body: &str) {
    #[cfg(feature = "notifications")]
    {
        notify_rust::Notification::new()
            .summary("URL Shortener")
            .body(body)
            .show()
            .expect("Failed to show notification.");
    }
}

fn main() {
    use std::env;

    let set_to_clipboard = env::args()
        .skip(1)
        .any(|arg| arg == SET_TO_CLIPBOARD_ARGUMENT_NAME);

    let get_from_clipboard = env::args()
        .skip(1)
        .any(|arg| arg == GET_FROM_CLIPBOARD_ARGUMENT_NAME);

    let disable_notifications = env::args().skip(1).any(|arg| arg == DISABLE_NOTIFICATIONS);

    let long_links: Vec<String> = if get_from_clipboard {
        match cli_clipboard::get_contents() {
            Ok(clipboard_content) => clipboard_content
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
            Err(e) => {
                eprintln!("Failed to read clipboard content: {:?}", e);

                return;
            }
        }
    } else {
        env::args()
            .skip(1)
            .filter(|arg| {
                arg != SET_TO_CLIPBOARD_ARGUMENT_NAME && arg != GET_FROM_CLIPBOARD_ARGUMENT_NAME
            })
            .collect()
    };

    let shortened_links = long_links
        .iter()
        .map(shorten_one)
        .collect::<Result<Vec<String>, urlshortener::providers::ProviderError>>()
        .expect("Failed to shorten links.");

    if set_to_clipboard {
        match cli_clipboard::set_contents(shortened_links.join("\n")) {
            Ok(_) => {
                println!("Shortened links copied to clipboard.");

                if !disable_notifications {
                    let count = shortened_links.len();
                    notify(&format!("Shortened {count} link(s) copied to clipboard."))
                }
            }
            Err(e) => eprintln!("Failed to copy shortened links to clipboard: {:?}", e),
        }
    } else {
        for (long, short) in long_links.iter().zip(shortened_links.iter()) {
            println!("{} -> {}", long, short);
        }

        if !disable_notifications {
            let count = shortened_links.len();
            notify(&format!("Shortened {count} link(s) successfully."))
        }
    }
}
