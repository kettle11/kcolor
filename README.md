# kcolor
A Rust library for handling color. (Work in progress)

The primary goal of this library is to provide a simple way to handle colors when the output color space of a monitor is not known at compile time.

sRGB has long been the standard, but many modern devices support wider color gamuts and it's no longer appropriate to simply assume the monitor's color space is sRGB.

Let's write software that supports better colors!

Right now this library is very much a work in progress intended for my personal use. Likely it is full of issues and will change frequently.
