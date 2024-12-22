# BeaconCalculator

A library for finding the best approximation of a color using a Minecraft beacon.

# But why?

There are already some websites that do this, like the [Minecraft Wiki](https://minecraft.wiki/w/Beacon) and [Minecraft.tools](https://minecraft.tools/en/beacon-color.php).
But I wanted a solution which can be automated and something that I can embed in my own website. So here it is!

# Quick example

Finding the best approximation of `#FE2C54`:

```rust
use beacon_calculator::find_combination_default;

find_combination_default([254, 44, 84]);

```

Returns:

```rust
Some(Panes {
    panes: [
        "pink",
        "magenta",
        "orange",
        "pink",
        "pink",
        "red",
    ],
    distance: 7.9557647705078125,
    color: PreciseRGB {
        red: 208.5,
        green: 89.90625,
        blue: 95.78125,
    },
})
```

For further information check out the docs.

# Optional Features

- `serde`: Derives Serialize and Deserialize for custom datatypes

# Disclaimer

This project is not affiliated with, endorsed by, or associated with Microsoft or Mojang.
