# libwisp-rs
A Rust library that exposes high-level APIs to get HTTP sites via [Wisp](https://github.com/MercuryWorkshop/wisp-protocol). (Wisp credits: MercuryWorkshop)

<sub>first real rust project, stuff may be messy, the code wasn't meant to be readable, just be easy-to-use for whatever dev wants to use this library</sub>

# THERE IS NO HTTPS URL SUPPORT IN LIBWISP-RS AT THE MOMENT <br /> HTTPS IS PLANNED FOR LIBWISP-RS 1.0.0
<sub>wss wisp servers still work, you just have to keep it HTTP-only when running a `WispHTTPRequest()`</sub>

## Installation
`cargo add libwisp`

## Building & Usage
For Usage, see `docs/` and `examples/`

For Building:
```
cargo build --release
```
## FAQ
- Q: How do I use an HTTPS site with this? {target_url} is saying its HTTP over HTTPS
- A: You can't, at least, not yet with libwisp-rs 0.1.0, as it lacks HTTPS support. This is a planned feature for libwisp-rs 1.0.0

- Q: How do I do ___ ?
- A: See the documentation and examples

- Q: I have a bug! Where do I report it?
- A: Using the GitHub issues tab

- Q: I have a change I'd like to make
- A: Use the GitHub pull request feature

## Credits
- kxtzownsu - writing libwisp-rs, fact checking documentation
- chatgpt - doing documentation (LOL)
- MercuryWorkshop - making the [Wisp protocol](https://github.com/MercuryWorkshop/wisp-protocol)
- ading2210 - hosting `wss://wisp.mercurywork.shop` which is used in examples
- scaratek - hosting `wss://nebulaservices.org/wisp/` which is used in examples
