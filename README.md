# timens-rs

Simple and efficient timestamp representation. The main objective being
interoperability with OCaml [Core_kernel.Time_ns](https://ocaml.janestreet.com/ocaml-core/v0.13/doc/core_kernel/Core_kernel/Time_ns/index.html).

[![Build Status](https://github.com/LaurentMazare/timens-rs/workflows/Continuous%20integration/badge.svg)](https://github.com/LaurentMazare/timens-rs/actions)
[![Latest version](https://img.shields.io/crates/v/timens.svg)](https://crates.io/crates/timens)
[![Documentation](https://docs.rs/timens/badge.svg)](https://docs.rs/timens)
![License](https://img.shields.io/crates/l/timens.svg)

A significant part of the code has been adapted from the OCaml [Core_kernel](https://github.com/janestreet/core_kernel) implementation.

The set of supported timezones can be filtered using the `TIMENS_TZ_FILTER`
environment variable, e.g.:
```bash
TIMENS_TZ_FILTER="London|New_York|Hong_Kong|Tokyo|GMT" cargo build
```
