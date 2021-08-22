# Moonhack-Rust edition

small project to create a game to show young people how to create simple Games in Rust.

see [CoderDojo - Linz](http://linz.coderdojo.net)

## building

taken from [`Unofficial Bevy Cheatbook`](https://bevy-cheatbook.github.io/platforms/wasm/wasm-pack.html)
### Windows/Linux

```cargo run```

or  
```cargo build```

### WASM

Setup and build:

```
rustup target add wasm32-unknown-unknown

cargo install wasm-pack
```

if installing wasm-pack did not work there is also a downloader for a pre-built binary (windows) at https://rustwasm.github.io/wasm-pack/installer/ .

Building can be done with 
```
wasm-pack build --target web --release
```

this builds needed files into a folder named `pkg`.

After that - please copy the `index.html` from the `web` folder and the whole `assets`-folder into `pkg` and deploy the whole thing on a webserver.

Have fun playing the game :)