#!/bin/bash

cargo install --path . -f
mkdir -p ~/.config/illumination/themes/default
curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.15.10/highlight.min.js > ~/.config/illumination/themes/default/hljs.min.js
curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js > ~/.config/illumination/themes/default/hljs-rust.js
curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.15.10/styles/default.min.css > ~/.config/illumination/themes/default/hljs.min.css
curl -X GET https://gist.githubusercontent.com/ryangray/1882525/raw/2a6e53f645b960f0bed16d686ba3df36505f839f/buttondown.css > ~/.config/illumination/themes/default/style.css
curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/github.min.css > ~/.config/illumination/themes/github.css