## tes3

A library for working with TES3 content.

Currently supports reading and writing of all `.esp`, `.esm`, `.nif`, `.kf` structures.

This library is still very much in-progress! At the moment it does little more than expose the core game structures for editing. Code quality or architecture may be questionable.

The APIs here should be considered unstable, struct/field names may be changed on a whim if I think of something better. If you're depending on this library for your own project it would be a good idea to pin to a specific commit.

The plan is to eventually stablize a useful API (with additional libs like `bsa`, `ini`, etc) and finally publish to `crates.io`. Not quite there yet though!

### Usage

This crate requires a relatively up-to-date **nightly** toolchain to compile. Partly to enable some performance improvements (SIMD), but mainly just because it's a hobby project for my own use and I like the syntax sugar conveniences. ;)

`cargo.toml`
```toml
[dependencies.tes3]
git = "https://github.com/Greatness7/tes3"
rev = "870b330"
default-features = false
features = ["esp"]  # add "nif" only if you need it
```

`main.rs`
```rs
use tes3::esp::{Plugin, Npc};

fn main() -> std::io::Result<()> {
    let mut plugin = Plugin::new();
    plugin.load_path("Path/To/Morrowind.esm")?;

    for object in plugin.objects_of_type::<Npc>() {
        if object.id == "fargoth" {
            println!("{object:#?}");
        }
    }

    Ok(())
}

```
