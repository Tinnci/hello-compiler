// Venus 编译器前端入口点
fn main() {
    use vil::frontend::parse_vil_file;
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <vil文件路径>", args[0]);
        std::process::exit(1);
    }

    let filepath = &args[1];
    match parse_vil_file(filepath) {
        Ok(module) => {
            println!("{}", module.borrow());
        }
        Err(e) => {
            eprintln!("解析错误: {}", e);
            std::process::exit(1);
        }
    }
}
