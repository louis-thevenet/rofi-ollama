# rofi-ollama
A [rofi](https://github.com/davatorium/rofi) plugin that asks [ollama](https://github.com/ollama/ollama) about available models to quickly start new conversations.

It only works with kitty for the moment, I am still looking for a general way of opnening terminal emulators. You can **open an issue** for other terminals and I'll add it. 

# Example
https://github.com/louis-thevenet/rofi-ollama/assets/55986107/46c66a7c-4d3a-4679-bcca-14798c955f3e

# Installation
## Via package manager
  The plugin is only packaged for Nix. [See](https://github.com/louis-thevenet/nixos-config/blob/2c745bbaf16ae9ec1d105daefe9c7b5cd544e9cf/home/louis/optional/wayland-wm/rofi-ollama/default.nix)

## From source
### Nix
To get a working dev shell via `nix`:

    nix develop

### Others
You will need `cargo` + some dependencies:
- `pkg-config`
- `openssl`
- `glib`
- `pango`
- `cairo`

### Build the plugin
For unreleased versions of `rofi`, you will need to `export ROFI_PLUGIN_PATH=$(pwd)/target/release`.
Then:

    cargo build --release

For local testing/usage:

      export ROFI_PLUGIN_PATH=PATH/TO/rofi-ollama/target/release/   
      rofi -modi ollama -show ollama

Or directly install the plugin:

      mv ./target/release/librofi_ollama.so /PATH/TO/ROFI/lib/rofi/librofi_ollama.so
