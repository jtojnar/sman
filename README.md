# sman

> I started using using `man --where --all $page | fzf | xargs man` instead of this so this is no longer maintained.

Utility that lists the sections requested term is available in and allows
user to choose. Kind of like interactive `apropos`.

The idea comes from the OpenSuse’s man-db, which is patched to do this.

## Usage

![sman in use](usage.png)

Just call it with names of a manual pages like  `sman open grep …`. They will be successively open.

## Debugging

You can run the tool like `env RUST_LOG=debug sman grep` to see information about scanned paths.

## Credits

Built with [Rust](https://www.rust-lang.org/) and [ncurses](https://www.gnu.org/software/ncurses/ncurses.html) (using [Cursive](https://github.com/gyscos/Cursive) library).
