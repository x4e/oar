# Oar

Terminal based text editor.

At the moment it can act as a simple pager, no text editing yet.
Just a public repo to document my (very slow) progress.

## Why?

I prefer terminal based text editors to graphical IDEs, but sadly vi is poorly designed legacy software, and none of it's numerous forks help this situation.

Most current text editors seem to force their users into using non ergonomic layouts, often with [very bad reasoning](https://catonmat.net/why-vim-uses-hjkl-as-arrow-keys).
Not only are these layouts bad for qwerty users, they are pretty much impossible to use on alternative layouts (such as colemak).

Additionally I hope to reduce the bloat that terminal text editors contain.
For example, nvim seems to natively support both lua and vimscript, both of which are terrible.

## Design

* Written in rust
* Single buffer (? maybe - I feel like multiple buffers are not necessary and should be provided by the terminal instead)
* Key mapping
* .editorconfig support
* Command system
