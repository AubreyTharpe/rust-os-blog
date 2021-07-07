This is a follow along of a blog series for writing an operating system in rust.

If you're interested in doing the same, cheers! Follow along here for the latest version.

https://os.phil-opp.com/

# Thoughts and Notes
## Post 1 - A Freestanding Rust Binary

An interesting read and set up.

- The rust extention for vs code is not happy about using baremetal rust. It doesn't seem to accomodate for the `[no_std]` attribute. Which, yeah, that's kind of niche. It complains about overwritting the panic from std which was necessary after disabling the standard library. It also doesn't seem to understand the `core` crate.
- It builds despite the extensions false negatives. Not much there but the beginnings of life has started. The kernel builds.