# urlshortener-cli
[![](https://meritbadge.herokuapp.com/urlshortener-cli)](https://crates.io/crates/urlshortener-cli)
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


## License

This project is [licensed under the MIT license](./LICENSE).
