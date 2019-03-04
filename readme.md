# linkr

#### The self-hosted solution for url shortening or prettifying.

> I made linkr because I wanted a self-hosted solution for url shortening but all
I found were implementations in PHP. The end goal is something as robust as existing solutions but
written in safe Rust, with an emphasis on RESTful APIs first.

## Features
 - Multi-threaded and secure as it uses [Rocket](https://rocket.rs)
 - Support for multiple identity providers and authorization/authentication modes
 - Easy to set up
 - Administrative endpoints to manage links
 - Built-in per-link statistics
 - Memory safe: Written in [Rust](https://rust-lang.org)

## Examples

With no authorization:
```bash
# Creating a link
curl -XPOST -d "origin=hello&dest=https://google.com&password=potato" your-domain.com/api/link

# Deleting a link
curl -XDELETE -d "origin=hello&password=potato" your-domain/api/link
```

Pre-shared key:
```bash
# Get token to use

```

## Getting started

Please see the [docs](docs/getting-started.md) to learn how to get started. 
