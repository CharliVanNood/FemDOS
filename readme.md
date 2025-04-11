# FemDOS
FemDOS is my hobby operating system, it's based on the operating system in rust tutorial. I followed the first part and left off from there, now I'm trying to make it into a full operating system. Why? I was bored.  
For help please read the [wiki](https://github.com/CharliVanNood/FemDOS/wiki)

# Features
- FemC is the built in programming language for FemDOS, it's an interperated language initially based on python syntax but now more towards C. The name FemC should be pronounced kinda like fancy.

# Compiling
- Make sure you have Rustup installed, if not install it like so:  
Windows: `winget install Rustlang.Rustup`  
Arch: `sudo pacman -S rustup`  
Ubuntu: `sudo apt install rustc`  
- Install Rust Nightly  
`rustup install nightly`
- Set nightly as default in this project  
`rustup override set nightly`  
`rustup component add rust-src --toolchain nightly`  
`rustup component add llvm-tools-preview --toolchain nightly`  
`rustup component add rust-src`  
- Install bootimage  
`cargo install cargo-xbuild`  
`cargo install bootimage`  

- Install QEMU (virtual machine, after compiling it gets called directly)  
Win32: https://qemu.weilnetz.de/w32/  
Win64: https://qemu.weilnetz.de/w64/  
Arch: `sudo pacman -S qemu` (system, not base)  
Ubuntu: `sudo apt install qemu-system`  

## Continue if you're on Windows
- Add QEMU to the enviroment variables  
First find the path, normally this is in `C:\Program Files\qemu`
In powershell you could run the command `[System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\qemu", [System.EnvironmentVariableTarget]::Machine)`  
Or just open the enviroment variables and add `C:\Program Files\qemu` to `Path`

# Setting up Qemu
- Create disk image  
`qemu-img create -f raw hdd.img 1024M`

# Running
- For compiling the project youself use  
`cargo run`  
