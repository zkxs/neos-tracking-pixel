# Neos Tracking Pixel

A proof-of-concept implementation of a tracking pixel.

## What is this?
This program generates an RGB PNG where the bytes of the color are the bytes of your IP address. Any remaining bytes needed to complete the last pixel are zeroed. The image is hosted here:
`http://0.0.0.0:3033/pixel/randomStringGoesHere/a.png`

The `randomStringGoesHere` path parameter is intended for cache-busting, but also demonstrates the ability to exfiltrate arbitrary data.

## Why would you do this?
This PoC is intended to demonstrate the risks of allowing images to be loaded on your behalf, without you having to opt-in.

## How do I run it?
Build command: `cargo build --color=always --workspace --all-targets --release`

Prebuilt binaries: [latest release](https://github.com/zkxs/neos-tracking-pixel/releases/latest)

Then simply run the generated executable. There are no configuration options.

