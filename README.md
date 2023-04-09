Hi !

This is a minimal concept program for making async calls with the use of newest standard implemention of async traits.
It uses snippets of code from the qedk/ethrs lib (rust) with some minor adaptations for the use of async traits.

---

To succesfully build an run this program is required the use of the latest Rust nightly version.
Running this command will install the nightly version of the tools (cargo, rustc, and so on). 
Also, it will switch the corresponding commands to use the nightly version.

$ rustup default nightly

To build and run the program type:
$ cargo run

---

After the test is completed you go back to the stable version, issue the following command:

$ rustup default stable

---

The links below offer some information on the async_traits 

Async functions in traits finally here!
https://www.google.com/search?channel=fs&client=ubuntu-sn&q=async+trait#fpstate=ive&vld=cid:b633a67f,vid:LRQP_Xwr0Wc

Async fn in trait MVP comes to nightly
https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html

QEDK / ethrs - An opinionated and blazing-fast crate for interacting with the EVM 
https://github.com/QEDK/ethrs/tree/master

Enjoy !
