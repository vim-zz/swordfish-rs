use swordfishlib;

#[test]
fn it_adds_two() {
    let screenplay = r###"
    - !clear
    - !write {msec: 0, color: green, text:  "$ "}
    - !write {msec: 0, text:    "i am going to list this dir"}
    - !wait {msec: 0}
    - !erase {msec: 0, by_chars: xxxxxxxxxxxxxxxxxxxxxxxxxxx }
    - !wait {msec: 0}
    - !write {msec: 0, text: "echo swordfish"}
    - !wait {msec: 0}
    - !execute {line: echo swordfish}
    - !wait {msec: 0}
    - !write {msec: 0, color: green, text:  "$ "}
    - !write {msec: 0, text: "bye, press any key..."}
    "###;

    let commands = swordfishlib::from_yaml(&screenplay).unwrap();
    swordfishlib::execute(commands).unwrap();
}