#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;

// defined host functions
mod host_extern {
  use wasmedge_quickjs::{Context, JsFn, JsValue};

  #[link(wasm_import_module = "env")]
  extern "C" {
      pub fn transfer(from: i32, to: i32, amount: i32) -> bool;
      pub fn balance(address: i32) -> i32;
  }

  pub struct BalanceFn;
  impl JsFn for BalanceFn {
      fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
          if let Some(JsValue::Int(address)) = argv.get(0) {
              unsafe {
                  let r = balance(*address);
                  r.into()
              }
          } else {
              ctx.throw_type_error("'address' is not a int").into()
          }
      }
  }

  pub struct TransferFn;
  impl JsFn for TransferFn {
      fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
          if let Some(JsValue::Int(from)) = argv.get(0) {
            if let Some(JsValue::Int(to)) = argv.get(1) {
              if let Some(JsValue::Int(amount)) = argv.get(2) {
                unsafe {
                    let r = transfer(*from, *to, *amount);
                    r.into()
                }
              } else {
                  ctx.throw_type_error("'amount' is not a int").into()
              }
            } else {
                ctx.throw_type_error("'to' is not a int").into()
            }
          } else {
              ctx.throw_type_error("'from' is not a int").into()
          }
      }
  }
}

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

        // add host functions into context
        let f = ctx.new_function::<host_extern::TransferFn>("transfer");
        ctx.get_global().set("transfer", f.into());
        //
        let f = ctx.new_function::<host_extern::BalanceFn>("balance");
        ctx.get_global().set("balance", f.into());
        //

        let (code, mut rest_arg) = args_parse();
        
        rest_arg.insert(0, code.clone());
        ctx.put_args(rest_arg);
        ctx.eval_module_str(code, "");

        ctx.js_loop().unwrap();
    });
}
