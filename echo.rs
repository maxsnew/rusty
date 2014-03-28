fn main() {
    let args = std::os::args();

    let print_newline = !(args.len() > 1 && args[1] == ~"-n");
    let start         = if print_newline { 1 } else { 2 };
    
    for i in range(start, args.len()) {
        print!("{} ", args[i]);
    };
    if print_newline {
        print("\n");
    }
}
