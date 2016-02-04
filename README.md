# zipper

ZIP file parser written in Rust.  Unzip works but zip is not supported.

Supported compression methods:
* Store (no compression)
* DEFLATE

Example:
```rust
extern crate zipper;

use std::path::Path;
use zipper::Archive;

fn main() {
    let path = Path::new("archive.zip");
    let archive = Archive::new(path).expect("new archive panic");
    archive.unzip().expect("unzip panic");
}
```
