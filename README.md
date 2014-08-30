This repository contains various code examples using Rust. 

#### Syntax Highlighting (VIM)

In order to get syntax highlighting for VIM, copy contents within `src/etc/vim` [here](https://github.com/rust-lang/rust) into `~/.vim`.

Then edit your `.vimrc` as seen below:

    filetype plugin indent on
    let g:no_rust_conceal = 1