mkdir -p .local/share/nvim/site/plugin/
mv neovim/illumination .local/share/nvim/site/plugin/illumination.vim
mkdir -p themes/default

curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.15.10/highlight.min.js > themes/default/hljs.min.js
curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js > themes/default/hljs-rust.js
curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.15.10/styles/default.min.css > themes/default/hljs.min.css
curl -X GET https://gist.githubusercontent.com/ryangray/1882525/raw/2a6e53f645b960f0bed16d686ba3df36505f839f/buttondown.css > themes/default/style.css
curl -X GET https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/github.min.css > themes/github.css
