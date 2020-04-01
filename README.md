# Github Actions (Rust)

This repository contains several different, but related pieces. It includes
a set of crates for building Github Actions in Rust, a set of Github Actions
built in Rust and an example of CI and Javascript bootstrapping for executing
Rust actions.

## Toolkit

The [`actions-toolkit`][actions-toolkit] crate provides common functionality for building
Github Actions in Rust. It is broken into sub-crates for more granular
usage.

## Actions

### [`wait`](./src/action/wait.rs)

The `wait` action is a sample action that accepts a number of milliseconds
as an input parameter and sleeps for that amount of time.

This action is sample code to prove functionality and will likely be removed
in the future.

```yaml
name: CI
on: push
jobs:
  wait:
    runs-on: ubuntu-latest
    steps:
      - uses: kjvalencik/actions/run/wait@master
        with:
          milliseconds: '10000'
```

## Releases

Rust Github Actions are released as pre-compiled binaries for each of the
platforms supported: Linux, Windows, and macOS. The [release workflow][workflow]
looks for version changes in `Cargo.toml` and creates new releases with
the built artifacts.

## Bootstrapping

Github Actions only support Javascript and Docker. In many cases, it may be
preferable to use a Docker Action. However, Docker Actions are limited to use
on Linux and do not have access to the action `$HOME` directory.

Instead of using Docker, a thin Javascript action can be used to download,
cache, and execute a binary.

The [`./run/index.ts`](./run/index.ts) file implements a generic bootstrap
action. The file reads the current version from `Cargo.toml` and downloads
the binary from [releases][releases].

### Sub-actions

Each action provided by this repository is specified in a sub-directory of
[`./run`](./run). These directories only include an `action.yml` to describe
the action and an `index.js` to load the bootstrap code. The bootstrap
action will provide the current directory as an argument to the action
binary. For example, `./action wait`. This allows multiple actions to
effectively be provided by a single binary, decreasing overall size.

[actions-toolkit]: ./crates/toolkit
[releases]: https://github.com/kjvalencik/actions/releases
[workflow]: ./.github/workflows/release.yaml
