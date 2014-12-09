/*
    >	++ptr;
    <	--ptr;
    +	++*ptr;
    -	--*ptr;
    .	putchar(*ptr);
    ,	*ptr = getchar();
    [	while (*ptr) {
    ]	}
*/

use std::os;

extern {
    fn getchar() -> i8;
}

fn get_val_of_ptr(val: &Vec<i8>, ptr: uint) -> i8 {
    if ptr < val.len() {
        val[ptr]
    } else {
        0
    }
}

fn set_val_of_ptr(val: &mut Vec<i8>, ptr: uint, v: i8){
    if ptr >= val.len() {
        for _ in range(0, ptr + 1 - val.len()) {
            val.push(0);
        }
    }
    val[ptr] = v;
}

// inc (if x==1) or dec (if x==-1)
fn inc_val_of_ptr(val: &mut Vec<i8>, ptr: uint, x: i8) {
    let x = get_val_of_ptr(val, ptr) + x;
    set_val_of_ptr(val, ptr, x);
}

#[deriving(Show)]
struct LoopState {
    from: uint,
    to: Option<uint>,
}

impl Copy for LoopState {}

impl LoopState {
    fn new(from: uint, to: Option<uint>) -> LoopState {
        LoopState { from: from, to: to }
    }
}

fn init_loop_states(states: &mut Vec<LoopState>, code: &str) {
    let mut stack:  Vec<LoopState> = vec![];
    for (i,c) in code.chars().enumerate() {
        match c {
            '[' => { stack.push(LoopState::new(i,None)); }
            ']' => {
                    if stack.len() > 0 {
                        let last = stack.len() - 1;
                        stack[last].to = Some(i);
                        states.push(stack.pop().unwrap());
                    } else {
                        note_char_at(code, i);
                        panic!("no `[` match this `]` at pos {}", i);
                    }
                }
            _ => { }
        };
    }
    for state in stack.iter() {
        if state.to.is_none() {
            note_char_at(code, state.from);
            panic!("no `]` match this `[` at pos {}", state.from);
        }
    }
    /*
    // for debug, print all loop states
    for state in states.iter() {
        println!("{}", state);
    }
    */
}

fn note_char_at(code: &str, index: uint) {
    println!("\n{}", code);
    for _ in range(0, index) {
        print!(" ");
    }
    println!("^");
}

fn match_loop_states(states: &mut Vec<LoopState>, index: uint) -> uint {
    for state in states.iter() {
        if state.from == index {
            return state.to.unwrap();
        } else if state.to.unwrap() == index {
            return state.from;
        }
    }
    panic!("no match loop state at pos {}", index);
}

fn main() {
    let args = os::args();
    if args.len() < 2 || args[1].len() == 0 {
        println!("Usage: bfc <code>\n");
        return;
    }
    let mut ptr: uint = 0;
    let mut val: Vec<i8> = vec![];
    let mut loop_states: Vec<LoopState> = vec![];
    let code = args[1].as_slice();
    init_loop_states(&mut loop_states, code);
    let mut ip = 0u; // Instruction pointer
    loop {
        if ip >= code.len() {
            break;
        }
        let op = code.char_at(ip);
        match op {
            '>' => ptr += 1,
            '<' => {
                if ptr == 0 {
                    note_char_at(code, ip);
                    panic!("can't < any more, pos {}", ip);
                }
                ptr -= 1;
            }
            '+' => inc_val_of_ptr(&mut val, ptr, 1),
            '-' => inc_val_of_ptr(&mut val, ptr, -1),
            '.' => println!("{}", get_val_of_ptr(&val, ptr) as u8 as char),
            ',' => {
                unsafe { set_val_of_ptr(&mut val, ptr, getchar() as i8); }
            }
            '[' => {
                if get_val_of_ptr(&val, ptr) == 0 {
                    ip = match_loop_states(&mut loop_states, ip) + 1;
                    continue;
                }
            }
            ']' => {
                ip = match_loop_states(&mut loop_states, ip);
                continue;
            }
            'v' => println!("{}", get_val_of_ptr(&val, ptr)),
            _ => {}
        }
        ip += 1;
    }
    // println!("{}", val); // for debug, print all ptr values
}
