#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;

#[macro_use]
extern crate user_lib;
use user_lib::*;
use user_lib::console::getchar;

const LF: u8 = 0x0au8;  // \n
const CR: u8 = 0x0du8;  // \r
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;
const LINE_START: &str = ">> ";

#[no_mangle]
fn main() -> isize {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!("{}", LINE_START);
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");

                if !line.is_empty() {
                    
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        // line.as_str(), str 结尾没有结束符 \0，手动补上
                        line.push('\0');
                        if exec(line.as_str(), &[]) < 0 {
                            println!("Error when executing!");
                            return -4;
                        }
                        unreachable!();
                    } else {
                        assert!(pid > 0);
                        let mut exit_code: isize = 0;
                        let exit_pid = waitpid(pid as usize, &mut exit_code);
                        assert_eq!(pid, exit_pid);
                        println!(
                            "Shell: Process {} exited with code {}",
                            pid, exit_code
                        );
                    }
                    line.clear();
                }

                print!("{}", LINE_START);
                line.clear();
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
    0
}
