use swordfishlib;

#[test]
fn play_commands() {
    let screenplay = r###"
    - !clear
    - !title {text: "title here"}
    - !prompt {color: bright_green, text:  "$"}
    - !write {msec: 0, color: blue, text:  "$ "}
    - !write {msec: 0, text:    "i am going to list this dir"}
    - !wait {msec: 0}
    - !erase {msec: 0, by_chars: xxxxxxxxxxxxxxxxxxxxxxxxxxx, amount: 5 }
    - !wait {msec: 0}
    - !write {msec: 0, text: "echo swordfish"}
    - !wait {msec: 0}
    - !execute {line: echo swordfish}
    - !wait {msec: 0}
    - !new_line
    - !write {msec: 0, text: "bye, press any key..."}
    - !turbo {by: 1}
    "###;

    let commands = swordfishlib::from_yaml(&screenplay).unwrap();
    let lines = screenplay.lines().collect::<Vec<_>>().len();
    assert_eq!(commands.len(), lines - 2);
    swordfishlib::execute(commands).unwrap();
}

#[test]
fn play_wrong_commands() {
    let screenplay = r###"
    - !no_command_like_this_one
    "###;

    let commands = swordfishlib::from_yaml(&screenplay);
    assert!(commands.is_err());
}

#[test]
fn play_wrong_command_arg() {
    let screenplay = r###"
    - !wait {no_arg_like_this_one: 0}
    "###;

    let commands = swordfishlib::from_yaml(&screenplay);
    assert!(commands.is_err());
}