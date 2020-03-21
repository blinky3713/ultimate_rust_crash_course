# Ultimate Rust Crash Course

This is the companion repository for the [Ultimate Rust Crash Course] published online.  You will
get the most out of this training experience by purchasing a license or subscribing to the course
and watching the course videos while running through the exercises in this repository.

[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course

Watching is fun, but you will learn a lot more if you dig in and do some coding!
This repository is for you hands-on-learners who are ready to roll.

I use macOS, and that is what I developed this course on.  Everything _ought_ to work similarly on major Linux
distributions and Windows. Please [contact me](mailto:nathan.stocks@gmail.com) ASAP if you have trouble with anything on
this page.


## Install Rust

Rust is required for this course!  The latest stable version is always recommended.

- Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started`
   button and follow the instructions to install Rust for your operating system.
   - Please DO NOT install rust via some other package manager.  It will probably be a version that is _really old_.

You should get somewhat similar output if you run commands like the ones below (newer versions are okay).  If you 
already have an old version of Rust installed, then run `rustup update` to install a newer version.

```shell
$ rustc --version
rustc 1.42.0 (b8cedc004 2020-03-09)
$ cargo --version
cargo 1.42.0 (86334295e 2020-01-31)

```

- Clone or download this repository to your computer.

## Prepare Your Development Environment

Please do the following (see the [How To Learn Rust](https://github.com/CleanCut/rust_programming/blob/master/HowToLearnRust.md)
page for details on all of these)
- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking
- [ ] Choose one place to "find answers" and either introduce yourself (if it's a forum, IRC, etc.) or find the answer
      to one question you have.
- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new message`
  - `cd message`
  - `cargo run`
  - Edit `src/main.rs` and change the message.
  - `cargo run` again to see your new message.
- [ ] Check out the descriptions of the tools and books.

# Training!

Now you are ready for the training!  Go watch the [Ultimate Rust Crash Course] and come back here
for exercises.

### Resources

- Training by the instructor (Nathan Stocks) in the form of the [Ultimate Rust Crash Course]
- This Repository :tada:
- [How To Learn Rust](https://github.com/CleanCut/rust_a_crash_course/blob/master/HowToLearnRust.md)
- [The Rust Standard Library](https://doc.rust-lang.org/std/)

### Exercises

- [Exercise A - Variables](https://github.com/CleanCut/rust_programming/tree/master/exercise/a-variables)
- [Exercise B - Functions](https://github.com/CleanCut/rust_programming/tree/master/exercise/b-functions)
- [Exercise C - Simple Types](https://github.com/CleanCut/rust_programming/tree/master/exercise/c-simple-types)
- [Exercise D - Control Flow & Strings](https://github.com/CleanCut/rust_programming/tree/master/exercise/d-control-flow-strings)
- [Exercise E - Ownership & References](https://github.com/CleanCut/rust_programming/tree/master/exercise/e-ownership-references)
- [Exercise F - Structs & Traits](https://github.com/CleanCut/rust_programming/tree/master/exercise/f-structs-traits)
- [Exercise G - Collections & Enums](https://github.com/CleanCut/rust_programming/tree/master/exercise/g-collections-enums)
- [Exercise Z - Final Project](https://github.com/CleanCut/rust_programming/tree/master/exercise/z-final-project)