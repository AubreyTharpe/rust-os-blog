This is a follow along of a blog series for writing an operating system in rust.

If you're interested in doing the same, cheers! Follow along here for the latest version.

https://os.phil-opp.com/

# Thoughts and Notes
## Post 1 - [A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/)

An interesting read and set up.

- The rust extention for vs code is not happy about using baremetal rust. It doesn't seem to accomodate for the `[no_std]` attribute. Which, yeah, that's kind of niche. It complains about overwritting the panic from std which was necessary after disabling the standard library. It also doesn't seem to understand the `core` crate.
- It builds despite the extensions false negatives. Not much there but the beginnings of life has started. The kernel builds.

## Post 2 - [A Minimal Rust Kernel](https://os.phil-opp.com/minimal-rust-kernel/)

Loads more configuration and setting things up for the x86 target

- Added a flushed out target json file to build against. Added config options to make it the default target so no more manual `--target=...`.
- Added bootloader crate to dependencies and bootimage cargo binary. Both provided by the blog author to keep us from getting bogged down in the details of how to build a bootable disk and setting up the assembly and setup to get from bios to 64 bit mode. Future blog material for them which I look forward to going through as well.
- Gotta have nightly build for `build-std` feature. I lucked out and already had it dabbling in rocket.
- Lot of work for a hello world but a hello world from a baby OS makes it worth it.
- `cargo bootimage` <--- That's the magic so far. The bootable image hangs out at `./target/x86_64-rust_os/debug/bootimage-rust_os.bin`
- Windows does not play well with the .bin that's generated....
- Originally written on Ubuntu and for QEMU 
- VirtualBox has an issue working with .bin files. From the comments on the post other people have had similar issues. `VBoxManage convertfromraw` will convert a bin to something virtualbox can use as long as it's >1MB... The kernel is approximately 63kB. Which can be resized with `qemu-img resize`. Thank you to atultw on April 21 for all this helpful information regarding this. Unfortunately, I'm not looking to install more stuff on my windows machine at this time. So I'm going to keep writing and following along in windows while testing in a Ubuntu VM on the side that I already have setup for development. I find it silly to install QEMU just to resize it just so I can get it to work with virtualbox when my ubuntu vm will already have it installed and I can test it in there without any extra setup. Hence this repo for making it easier to pass the code between the two computers and to keep a copy for future me.