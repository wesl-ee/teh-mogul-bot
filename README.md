teh-mogul-bot
=============

A Discord bot exposing a locally-hosted version of `stable-diffusion`. Relies on
[stable-diffusion-webui's](https://github.com/AUTOMATIC1111/stable-diffusion-webui)
exposed REST API.

Usage
-----

First create a Discord bot and get its token.

Then install
[stable-diffusion-webui](https://github.com/AUTOMATIC1111/stable-diffusion-webui)
and run with `./webui.sh --listen --nowebui` on any machine. Now clone this
repository, build (as below) and copy `env.sample` to `.env`, changing the
parameters to match those of your bot and stable-diffusion-webui URI.

Building
--------

Install Rust using `rustup` or however you like. Then just do `cargo build`;
nothing fancy here.

For an environment with `nix` installed one can run `nix-shell` to bootstrap a
dev environment without the need to fumble with `rustup`.

License
-------

MIT License (available under /LICENSE)
