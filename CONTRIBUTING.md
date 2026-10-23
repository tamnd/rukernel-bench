# Contributing

This document tells you how to make a change. The design of the harness and the rules for a result are in [spec/19-rukernel-bench.md](https://github.com/tamnd/rukernel/blob/main/spec/19-rukernel-bench.md) in tamnd/rukernel.

## Before you start

Each milestone of rukernel needs parts of this harness. The milestone tracking issues have the label `kind/milestone`. Select an item and write a comment on the issue before you start.

## Run the checks

```
cargo xtask ci
```

This command runs `cargo fmt`, the prose rules, `cargo clippy`, `cargo test`, `cargo doc` and `rkb check`.

## The rules for a change

**Show the losses.** Do not remove a workload because rukernel is slow on it. A red row stays in the table until rukernel is fast enough.

**Measure on the reference fleet.** A result from a laptop is useful for a pull request. It does not go into the table.

**Match the machines.** Both kernels use the same firmware settings, microcode, mitigations and CPU frequency policy. A difference in the machine is a bug in the harness.

**Give the numbers.** Each claim has the command, the median of ten runs, the interquartile range and the machine.

**Each benchmark program is pinned.** A workload names its program by version and checksum. To move the version, make a commit that changes both.

## Style

**Rust.** `cargo fmt` sets the layout. A comment tells why, not what.

**Prose.** This rule applies to documents, commit messages, issues and pull requests. Use plain English. Use short sentences. Use the active voice. Do not use em dashes or en dashes. Do not use horizontal rules. Do not break a sentence across lines. `cargo xtask style` examines the dashes and the rules.

## Commits

Write the commit subject in the imperative mood. Start with the part that you change, for example `fio: pin the 3.41 release`. A maintainer merges with squash or rebase.
