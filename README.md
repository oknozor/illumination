# Nvim Illumination : A lightweight GTK live preview for your notes
---

Nvim Illumination is a simple Neovim plugin to render buffers in a WebKit2Gtk window. Since it uses the Neovim rpc-api to send buffer updates, **it will not work with vim**.

Note that unlike many note rendering plugins Illumination does not render files, it renders your buffer directly via nvim rpc api, it means you don't need to save the current buffer to render your notes, it is really live!  

Issues are welcome but Illumination is at a early stage of development, before submiting one please read the [Roadmap](#roadmap) section.  

![example screenshot](screenshots/demo.png)

## Quick start 
---

There is currently no build script, the simplest way to test the plugin is the following : 

1. Compile the rust binary : 

``` 
cargo build --release
```

2. Modify the bin path in [plugin/illumination.vim](plugin/illumination.md) :

```
let s:bin = 'path_to_this_repo/target/release/illumination'
```

3. Source the script and run it :
    - open some markdown file with nvim
    - source the launch script : `:source path_to_this_repo/plugin/illumination.vim`
    - run the following command `:Render`
    - to stop rendering : `:RenderStop` or just close your current nvim instance

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
    - [ ] autoscroll

