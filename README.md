# blockz

Blockz is an opinionated library that aims to make it a pleasure to develop
networked applications in Rust.

## Why Rust?

- bEcAuSe It'S a NeW lAnGuAgE
- iT iS bEtTeR tHaN c Or C++
- It WaS sTaRtEd By MoZiLlA, nOt UnLiKe Go, WhIcH wAs StArTeD bY tHe EvIl
CoRpOrAtIoN, gOoGlE

Just kidding. This is a TL;DR of my reasons:

- If it compiles, it will probably have business logic bugs at most. Very, very
slim chances of memory or concurrency issues. And that makes me feel great.
- The macro system is great. You can create some really cool stuff(looking at
you, [serde]). And that happens at compile-time, which means free
bug-inspection by the compiler and free performance.
- [Zero cost abstractions].
- Ease of deployment, once compiled. Compile for musl, ship in an alpine
container, enjoy. Oh, issues with OpenSSL? Just use [rustls].
- Great bang for your buck. I'm a student.

[serde]: https://serde.rs
[Zero cost abstractions]: https://boats.gitlab.io/blog/post/zero-cost-abstractions/
[rustls]: https://github.com/ctz/rustls
