fn main() {
    let args = std::os::args();

    let t = 
        if args.len() > 1 && args[1] == ~"-n" {
            (2, false)
        } else {
            (1, true)
        };
    
    match t {
        (start, print_newline) => {
            for i in range(start, args.len()) {
                print(args[i]);
                print(" ");
            };
            if print_newline {
                print("\n");
            }
        }
    }
}
