# 🦀 extract-links-rust

## Crates Used

- [error-chain](https://crates.io/crates/error-chain): For error handling in Rust.
- [reqwest](https://crates.io/crates/reqwest): HTTP client for making web requests.
- [select](https://crates.io/crates/select): A library to extract useful data from HTML documents, suitable for web scraping.
- [tokio](https://crates.io/crates/tokio): Runtime for asynchronous Rust programs.

## Project Description

This Rust project demonstrates how to extract links from a web page. It uses the `reqwest` crate to make HTTP requests, fetches the content of a webpage, and then utilizes the `select` crate for parsing and extracting links from the HTML document.

## Setup and Run

1. Clone the repository:

```bash
git clone https://github.com/codescalper/extract-links-rust.git
```

2. Navigate to the project directory:

```bash
cd extract-links-rust
```

3. Build and run the project with Cargo:

```bash
cargo run
```

## Where It Can Be Used

1. **Web Scraping**: This project can be used as a starting point for web scraping tasks where you need to extract links and data from web pages.
2. **Content Analysis**: It can be integrated into applications for analyzing and extracting links, titles, or other data from web articles, blogs, or news sites.

---

If you have any questions or suggestions, feel free to reach out on Twitter: [@mayanks_tw](https://twitter.com/mayanks_tw).

---

#### Output Image

![Output Image](https://cdn.discordapp.com/attachments/1150040438904979557/1169212781753552906/image.png)
