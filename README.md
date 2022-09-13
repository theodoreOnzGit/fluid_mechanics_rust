# Here's what i did to install the file

## development environment

I use cargo watch:

```powershell
cargo install cargo-watch
cargo watch -x run
```

### vim-lsp-settings
In neovim, vim-lsp-settings i use rust-analyzer.

However, when you run LspInstallServer, it says curl is
not working due to some security issues.

I had to download this in a unsecure manner.

However, to do that on windows, i had to use:

```cmd
@echo off

setlocal
curl -k -L -o "rust-analyzer-windows.gz" "https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-pc-windows-msvc.gz"
call "%~dp0\run_gzip.cmd" rust-analyzer-windows.gz

move rust-analyzer-windows rust-analyzer.exe
```

Basically it was found in the filepath:"

```powershell
C:\Users\$env:UserName\AppData\Local\nvim-data\plugged\vim-lsp-settings\installer\install-rust-analyzer.cmd
```
And i edited the curl file to include the -k option.

## cargo installation

I installed

```powershell
cargo add uom
```

## combining library and binary

https://dev.to/yjdoc2/make-a-combined-library-and-binary-project-in-rust-d4f

I followed the instructions above and changed the cargo.toml:

```toml
[[bin]]
name = "fluid_mechanics_rust"
path = "src/bin.rs"
```
i then added a lib.rs file in the following filepath
src/lib/lib.rs,
and added the following to cargo.toml

```toml
[lib]
name = "fluid_mechanics_rust"
path = "src/lib/lib.rs"
```
