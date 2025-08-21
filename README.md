# ASIMOV Luma Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-luma-module)](https://crates.io/crates/asimov-luma-module)
[![Documentation](https://docs.rs/asimov-luma-module/badge.svg)](https://docs.rs/asimov-luma-module)

[ASIMOV] module for integration with [lu.ma](https://lu.ma) service.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install luma -v
```

### Installation from Source Code

```bash
cargo install asimov-luma-module
```

## 👉 Examples

```bash
asimov-luma-fetcher "https://lu.ma/discover"
asimov-luma-fetcher "https://lu.ma/ai?k=t"
asimov-luma-fetcher "https://lu.ma/genai-collective?k=c"
asimov-luma-fetcher "https://lu.ma/sf?k=p"
asimov-luma-fetcher "https://lu.ma/ai"
asimov-luma-fetcher "https://lu.ma/sf"
```

```bash
asimov-luma-cataloger list-featured-calendars
asimov-luma-cataloger list-palces
asimov-luma-cataloger get-place-by-slug "sf"
asimov-luma-cataloger get-nearby-events
asimov-luma-cataloger get-nearby-events-for-category "ai"
asimov-luma-cataloger get-calendar-events "cal-47FMgklxaO96VYi"
asimov-luma-cataloger get-place-events "discplace-gCfX0s3E9Hgo3rG"
```

## ⚙ Configuration

This module requires no configuration.

## 📚 Reference

### Installed Binaries

- `asimov-luma-cataloger`
- `asimov-luma-fetcher`

### `asimov-luma-cataloger`

```
asimov-luma-cataloger

Usage: asimov-luma-cataloger [OPTIONS] <COMMAND>

Commands:
  list-featured-calendars         List featured calendars.
  list-categories                 List categories.
  list-places                     List places.
  get-category                    Get category info by slug.
  get-calendar                    Get calendar info by id.
  get-calendar-events             Get events for a calendar by id.
  get-place-by-id                 Get place info by id.
  get-place-by-slug               Get place info by slug.
  get-place-events                Get events for a place by id.
  get-event                       Get event info by id.
  get-nearby-events               Get nearby events.
  get-nearby-events-for-category  Get nearby events for category by slug.
  help                            Print this message or the help of the given subcommand(s)

Options:
  -d, --debug       Enable debugging output
      --license     Show license information
  -v, --verbose...  Enable verbose output (may be repeated for more verbosity)
  -V, --version     Print version information
  -h, --help        Print help
```

### `asimov-luma-fetcher`

```
asimov-luma-fetcher

Usage: asimov-luma-fetcher [OPTIONS] <RESOURCE>

Arguments:
  <RESOURCE>

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -n, --limit <COUNT>    The maximum number of resources to list
  -o, --output <FORMAT>  The output format
  -h, --help             Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-luma-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-luma-module&text=asimov-luma-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-luma-module&title=asimov-luma-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-luma-module&t=asimov-luma-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-luma-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-luma-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[Rust]: https://rust-lang.org
