[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=Z8QK6XU749JB2) 
[![Latest Version][crate-badge]][crate-link] 
[![docs][docs-badge]][docs-link]
![Lines of Code][loc-badge]
[![MIT][license-badge]][license-link] 


# Track Macro

A macro attribute that indicates a type that needs to be tracked and implements
[Trackable](LINK) and [TrackableMarker](LINK).

This attribute **should** be used with the [track](https://crates.io/crates/track) crate.  

# Examples
```rust
#[track]
pub struct Position1 {
    pub x: u32,
    pub y: u32
}

#[track(serialization = "Bincode")]
pub struct Position2 {
    pub x: u32,
    pub y: u32
}
```

[crate-badge]: https://img.shields.io/crates/v/track-macro.svg
[crate-link]: https://crates.io/crates/track-macro

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: ./docs/LICENSE

[docs-badge]: https://docs.rs/track-macro/badge.svg
[docs-link]: https://docs.rs/track-macro/

[loc-badge]: https://tokei.rs/b1/github/entity-sync-rs?category=code