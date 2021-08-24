# Crawlrs

A web crawling tool written in rust

# Usage

```sh
crawlrs 0.1.0

USAGE:
    crawlrs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    crawl
    help     Prints this message or the help of the given subcommand(s)
```

```sh
crawlrs-crawl 0.1.0

USAGE:
    crawlrs crawl [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -s, --same-domain    Only crawl pages from the same domain
        --stats          Include session stats in output
    -V, --version        Prints version information

OPTIONS:
    -o, --json <output>     output format
    -u, --urls <urls>...    The url(s) to start crawling from
```
