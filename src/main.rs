#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;

fn args_parse() -> (String, Vec<String>) {
    use argparse::ArgumentParser;
    let mut code = String::new();
    let mut res_args: Vec<String> = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut code)
            .add_argument("code", argparse::Store, "js code")
            .required();
        ap.refer(&mut res_args)
            .add_argument("arg", argparse::List, "arg");
        ap.parse_args_or_exit();
    }
    (code, res_args)
}

fn main() {
    use wasmedge_quickjs as q;
    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let (code, mut rest_arg) = args_parse();
        
        rest_arg.insert(0, code.clone());
        ctx.put_args(rest_arg);
        ctx.eval_module_str(code, "");

        ctx.js_loop().unwrap();
    });
}
