# somedevfox/rcxp
rcxp is a rewrite of mkxp by Ancurio from the ground up in Rust with the intent of being faster and more efficient.

mkxp is an amazing project as is, but it falls short because it uses C++, leading to a spot of frustration when attempting to expand upon its codebase.

rcxp seeks to avoid this problem entirely by using Rust, rather than C++.

## License
rcxp Project is licensed under GPLv3 license. (see LICENSE)

## Authors and contributors
Authors of Software:
 - Egor "somedevfox" Poleshko <somedevfox@gmail.com>
 - Matthew "Speak2Erase" Lyons <matthew@nowaffles.com>

## Building
To build rcxp, you will need these dependencies:
 - SDL2 (https://libsdl.org) - MSVC Build
 - Ruby (https://ruby-lang.org/en) - MinGW Build

After downloading dependencies above, please, move all .lib/.a files to:

Windows:
> `C:/Users/[USERNAME]/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/x86_64-pc-windows-msvc/lib`

Linux:
> [TODO]

Then, to BUILD rcxp, execute:
> `cargo build`

To RUN rcxp, execute:
> `cargo run`