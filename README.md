# Swordfish 

Cli tool for screen typing pre defined scripts.

Example screenplay file:

```yaml
- !clear
- !write {msec: 0, color: green, text:  "$ "}
- !write {msec: 20, text:    "i am going to list this dir"}
- !wait {msec: 1000}
- !erase {msec: 20, by_chars: xxxxxxxxxxxxxxxxxxxxxxxxxxx }
- !wait {msec: 1000}
- !write {msec: 20, text: ls}
- !wait {msec: 1000}
- !execute {line: ls -la}
- !wait {msec: 3000}
- !write {msec: 1000, color: green, text:  "$ "}
- !write {msec: 20, text: "bye, press any key..."}
- !pause
```

## Quick start

Install 

```sh
cargo install swordfish-rs
```

## Usage

Create a screenplay file and run swordfish:

```sh
swordfish-rs path/to/file.yaml
```