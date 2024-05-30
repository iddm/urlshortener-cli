# urlshortener-cli
[![](https://meritbadge.herokuapp.com/urlshortener-cli)](https://crates.io/crates/urlshortener-cli)
[![](https://travis-ci.org/vityafx/urlshortener-cli.svg?branch=master)](https://travis-ci.org/vityafx/urlshortener-cli)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


A very simple urlshortener program written in Rust language. It uses [urlshortener](https://github.com/iddm/urlshortener-rs) rust library as data provider.


## Installing with `Cargo`

```
cargo install urlshortener-cli
```

## Usage
The interface is minimal as possible:

```
$ urlshortener-cli http://google.ru http://yandex.ru
http://google.ru -> https://is.gd/h5kR5r
http://yandex.ru -> https://is.gd/CifrPx
```

If your url has escaped characters just wrap it with quotes:

```
$ urlshortener-cli "https://www.google.ru/url?sa=t&rct=j&q=&esrc=s&source=web&cd=2&cad=rja&uact=8&ved=0ahUKEwiclOPp3OTRAhVyb5oKHUzyCl8QFggpMAE&url=http%3A%2F%2Ftest.tankionline.com%2F&usg=AFQjCNFIEFFpu2m_QofHelAXRK0JP4dLOQ&sig2=Fc6eFhPDqNgt5kZMzRWPIA"

https://www.google.ru/url?sa=t&rct=j&q=&esrc=s&source=web&cd=2&cad=rja&uact=8&ved=0ahUKEwiclOPp3OTRAhVyb5oKHUzyCl8QFggpMAE&url=http%3A%2F%2Ftest.tankionline.com%2F&usg=AFQjCNFIEFFpu2m_QofHelAXRK0JP4dLOQ&sig2=Fc6eFhPDqNgt5kZMzRWPIA -> https://is.gd/jIsDF1
```

The crate supports setting the clipboard content with the optional
`--set-to-clipboard` argument. The shortened links are then copied into
the clipboard for easier use.

```
$ urlshortener-cli --set-to-clipboard "https://google.com"
```

The crate also supports gets the links to shorten from the clipboard.
For this, use the `--get-from-clipboard` argument.

```
$ urlshortener-cli --get-from-clipboard
```

Note that getting the links from the clipboard may be messy on Linux if
the clipboard managers are used.

## License

This project is [licensed under the MIT license](./LICENSE).
