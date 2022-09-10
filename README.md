# swordfish-rs 

Cli tool for typing effect in Termainl. 

`sowrdfish` for screencasts and demos:

1. 💬 Describe what you are doing 
2. ⚡️ Run any terminal command and get their outputs to screen 
3. 🤖 Reproducable steps - iterate on the `screenplay` file till perfection
4. 😎 Mimick real person behaviour with realtime typing into terminal

![Swordfish hack scene](swordfish_hack_scene.gif)

## Demo

Example `screenplay.yaml` file:

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

Running `swordfish screenplay.yaml`:

![demo](demo.gif)

## Quick start

Install 

```sh
cargo install swordfish-rs
```

## Usage

Create a screenplay file and run swordfish:

```sh
swordfish path/to/file.yaml
```

### Commands

The follwoing comamnds are available:

#### Write 

Write text to the terimal.

| Argument | Type | Description |
| - | - | - |
|`text`| String | the text to type in the terimal, each charecter will be entered on by one with some delay |
|`msec`| Integer | delay between typed chars in milisec |
|`color` (optional)| String | text's color: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white` or a brighter variant, for example `bright_red` |
    
#### Erase 

Erase charecters to the left.

| Argument | Type | Description |
| - | - | - |
|`by_chars`| String | the amount of backspace is determind by the length of the provided text |
|`msec`| Integer | delay between individual backspaces in milisec |

#### Execute 

Execute shell commands or other applications and show their output.

| Argument | Type | Description |
| - | - | - |
|`line`| String | command line to execute, respects quoted arguments |

The output is presented, while the executed command itself will not show.

#### Wait 

| Argument | Type | Description |
| - | - | - |
|`msec`| Integer |  delay before next command in milisec |

#### Clear 

Clear screen command.

#### Pause 

Pause before next command and wait for user input (any key...)
