# swordfish-rs 

Cli tool for typing effect in Termainl for screencasts.

![Swordfish hack scene](swordfish_hack_scene.gif)

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

`text`: the text to type in the terimal, each charecter will be entered on by one with some delay
`msec`: delay between typed chars in milisec
`color` (optional): text's color 

Availave colors: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`.
All colors has a brighter variant, for example `bright_red`.
    
#### Erase 

Erase charecters to the left.

`by_chars`: the amount of backspace is determind by the length of the provided text 
`msec`: delay between individual backspaces in milisec

#### Execute 

Execute shell commands or other applications and show their output.

`line`: command line to execute, respects quoted arguments

The output is presented, while the executed command itself will not show.

#### Wait 

`msec`: delay before next command in milisec

#### Clear 

Clear screen command.

#### Pause 

Pause before next command and wait for user input (any key...)
