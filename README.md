# rukernel-bench

rukernel-bench is the benchmark harness of [rukernel](https://github.com/tamnd/rukernel). It measures rukernel and Linux 7.3 on the same machines and publishes each result. The table shows the losses with the wins.

The project is at milestone R0. The harness CLI, `rkb`, examines the machines and the workloads. The first bench table comes with milestone R4.

## The rules

[spec/19-rukernel-bench.md](https://github.com/tamnd/rukernel/blob/main/spec/19-rukernel-bench.md) gives the full rules. These are the most important:

1. Both kernels run on the same machine, with the same firmware, the same microcode and the same mitigations.
2. Each workload runs ten times or more on each kernel. The table gives the median and the interquartile range.
3. A Mann-Whitney test decides if a difference is real.
4. rukernel must meet the gate against the distribution configuration and against the tuned configuration of Linux.
5. A red row stays in the table. A faster row does not cancel a slower row.

## The gates

A gate is the largest loss against Linux that a class can have at 1.0.

| Class | Gate | Examples |
|---|---|---|
| entry | 5% | System calls, the vDSO, signals |
| sched | 5% (10% at p99) | Pipe round trips, hackbench, schbench |
| mem | 10% | Page faults, fork, mmap, will-it-scale |
| storage | 5% | fio on NVMe with io_uring |
| fsmeta | 10% | stat, git status, a kernel build |
| net | 5% (10% at p99) | netperf TCP_RR, iperf3, nginx |
| apps | 5% | PostgreSQL, Redis, compilers, games |

## Use

You need `rustup`. The toolchain file installs the correct Rust version.

```
cargo run --bin rkb -- help
cargo run --bin rkb -- check
cargo xtask ci
```

`rkb check` examines `machines.toml` and each workload. A workload with fewer than ten runs, or a gate wider than the gate of its class, is an error.

## Repository layout

| Path | Contents |
|---|---|
| `rkb/` | The harness CLI |
| `machines.toml` | The reference fleet |
| `configs/` | The Linux configurations and the rukernel profiles |
| `workloads/<class>/<name>/bench.toml` | One file for each workload |
| `xtask/` | The build checks |

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md). Report security problems as [SECURITY.md](SECURITY.md) tells you.

## License

The Apache License 2.0. Refer to [LICENSE](LICENSE). Each benchmark program keeps its own license.
