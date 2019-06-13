# urlshortener-cli
[![](https://meritbadge.herokuapp.com/urlshortener-cli)](https://crates.io/crates/urlshortener-cli)
[![](https://travis-ci.org/vityafx/urlshortener-dbus-daemon.svg?branch=master)](https://travis-ci.org/vityafx/urlshortener-cli)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


A very simple urlshortener program written in Rust language. It uses [urlshortener](https://github.com/vityafx/urlshortener-rs) rust library as data provider.


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

## License

This project is [licensed under the MIT license](./LICENSE).
