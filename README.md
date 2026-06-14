# ferrisbf

A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in Rust, mostly as a stepping stone before attempting to write a GBA emulator.

## Motivation

I just finished my second read of the Rust book ans started working through [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/), but I was feeling like it was time to attempt something on my own that was a bit more advanced. That's when I remembered that, when learning Go, I followed this really well written [tutorial by Thorsten Ball](https://thorstenball.com/blog/2017/01/04/a-virtual-brainfuck-machine-in-go/) about how to write a Brainfuck VM in Go (I also have his two books about writing a compiler and a interpreter and I can't recommend both enough).

## Performance

I am still not ready to work through more serious features (like adding a JIT), but I still managed to add the typical tricks that people usually use like collapsing instructions when parsing.

The Mandelbrot example program doubles as both a correctness check and a rough benchmark for all of this.

## Usage

Build it:

```sh
cargo build --release
```

Then run any `.bf` file in the examples folder:

```sh
./target/release/ferrisbf examples/hello.bf
```

or, without building first:

```sh
cargo run --release -- examples/hello.bf
```
