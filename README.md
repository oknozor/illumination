# Nvim Illumination : A lightweight GTK live preview for your notes
---

Nvim Illumination is a simple Neovim plugin to render buffers in a WebKit2Gtk window. Since it uses the Neovim rpc-api to send buffer updates, **it will not work with vim**.

Note that unlike many note rendering plugins Illumination does not render files, it renders your buffer directly via nvim rpc api, it means you don't need to save the current buffer to render your notes, it is really live!  

Issues are welcome but Illumination is at a early stage of development, before submiting one please read the [Roadmap](#roadmap) section.  

![example screenshot](screenshots/demo.png)

## Intallation
---

1. Compile the rust binary and add it to your path : 

```sh 
git clone  https://github.com/oknozor/illumination/ 
cd illumination
cargo install --path .
```

2. Copy `illumination.vim` in your runtime path (see `:help runtimepath`) :

### Usage

Within Neovim use the `:Render` command to start rendering and `:RenderStop` to quit Illumination.

## Debug

To debug Illumination start nvim in RPC mode on port `6666` : 

```
nvim test.md --listen 127.0.0.1:6666
```

Then run Illumination without cargo `--release` option. 

```
cd illumination
cargo run 
```

## Roadmap

---
- Build
    - [ ] build script
    - [ ] travis
- CSS
    - [ ] replace CDN with local CSS  
    - [ ] theme selection via config file
- Nvim integration
    - [ ] filetype 
    - [x] switching buffer
- Gtk
    - [ ] add pdf export
    - [ ] asciidoctor support
    - [ ] latex support
    - [x] autoscroll

