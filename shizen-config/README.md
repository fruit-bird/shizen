A configuration crate for specifying VST information in the [`shizen`](https://github.com/fruit-bird/shizen) crate

The information configured in the `shizen` table of the `Shizen.toml` VST
manifest is used to populate the necessary information about a VST

```toml
[shizen.metadata]
vendor = "Shizen Technologies"
categories = ["effect", "utility", "midi"]
plugin-type = "instrument"

[shizen.processing]
block-size = 1024
```

This configuration will also extract the package information
from the package `Cargo.toml` or the workspace `Cargo.toml`
if the package is part of a workspace
