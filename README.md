# Rust Driver Development Template (Windows)

This project can be used as a template for Driver Development in Windows without using MS Visual Studio and C# or C++ as the dominant language. 

As of now, I have not mappeed the Kernel Procedures provided by MS Docs into equivalent rust functions. You might have to do that yourself. 

But I will be working on that soon enough. 

## Steps to Run this project
- You need to have a nightly version of Rust toolchain (and not the standard one) as we use -Z pre-link attributes as part of our build process. 
- Run `cargo b` to build the Project. You will find the dll file in `target/x86_64-pc-windows-msvc`.
- Install cargo-make tool by running `cargo install --force cargo-make`
- Run `cargo make --profile production sign` to build driver and sign it for release.