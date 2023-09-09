# neofallout

Fallout 1/2 game engine in Rust

This is a personal project for rewriting the Fallout 1/2 game engine in Rust. It's based on the projects:

- [fallout2-ce](https://github.com/alexbatalov/fallout2-ce)
- [vault13](https://github.com/pingw33n/vault13)

The following steps will be made:

- [ ] rewrite main.cpp to Rust and fallout2-ce is fully playable
- [ ] rewrite source code from fallout2-ce to Rust -> Fallout 2 should be fully playable
- [ ] add support for loading maps and other data from Fallout 1 -> Fallout 1 should be fully playable by this game engine, too (like in [Fallout 1 in 2](https://github.com/rotators/Fo1in2))
- [ ] ensure modding and HighRes support based on Fallout 2 mods and for Fallout 1

I named this project **neofallout** because I want to take the fallout 1/2 game engine to a new level like neovim did to vim.
Therefore after the previous steps are implemented successfully:

- Mods for the new game engine are based on [WebAssemblies](https://webassembly.org/) by using something like [this](https://paulbutler.org/2021/calling-webassembly-from-rust/)
- Fallout 1 and 2 are loaded as WebAssemblies
- ... (more ideas will be added hopefully)

Contributions/Code Reviews are always welcome! I work on this project in my spare time, so don't expect that this project will be finished soon, if ever. And my answers on issues, pull requests etc. could take more than one day.
If you have findings from your Code Review, please open an issue with some meaningful comments!
Rewriting the code from C++ to Rust is done by

- transpiling C++ to Rust,
- adding FFI for bridging currently not transpiled parts of fallout2-ce.
  In such a way, I hope to slice the origin code into smaller pieces that can be rewritten in Rust.
