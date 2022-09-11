use swordfishlib;

#[test]
fn play_commands() {
    let screenplay = r###"
    - !clear
    - !prompt {color: green, text:  "$"}
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
    "###;

    let commands = swordfishlib::from_yaml(&screenplay).unwrap();
    assert_eq!(commands.len(), 13);
    swordfishlib::execute(commands).unwrap();
}