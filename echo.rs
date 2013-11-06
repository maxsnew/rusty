fn main() {
    let args = std::os::args();

    for i in range(1, args.len()) {
        print(format!("{:s} ", args[i]));
    }
    println("");
}