#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;

// defined host functions
mod host_extern {
  use wasmedge_quickjs::{Context, JsFn, JsValue};

  #[link(wasm_import_module = "env")]
  extern "C" {
      pub fn transfer(address_pointer: *const u8, address_length: i32, amount: f32);
      pub fn balance() -> f32;

      pub fn safe_transfer(address_pointer: *const u8, address_length: i32, amount: f32);
      pub fn hold(address_pointer: *const u8, address_length: i32, amount: f32);
      pub fn hold_return(address_pointer: *const u8, address_length: i32, amount: f32);
      pub fn contract_address(target_pointer: *const u8) -> i32;
      pub fn sender(target_pointer: *const u8) -> i32;
      pub fn call(contract_name_pointer: *const u8, contract_name_length: i32, nonce_pointer: *const u8, nonce_length: i32, function_name_pointer: *const u8, function_name_length: i32, parameters_pointer: *const u8, parameters_length: i32, target_pointer: *const u8, depth: i32) -> i32;
      pub fn delegatecall(contract_name_pointer: *const u8, contract_name_length: i32, nonce_pointer: *const u8, nonce_length: i32, function_name_pointer: *const u8, function_name_length: i32, parameters_pointer: *const u8, parameters_length: i32, target_pointer: *const u8, depth: i32) -> i32;
      pub fn randomish(target_pointer: *const u8) -> i32;
      pub fn ownership_transfer(address_pointer: *const u8, address_length: i32);
      pub fn version() -> i32;
      pub fn store_state(key_pointer: *const u8, key_length: i32, value_pointer: *const u8, value_length: i32);
      pub fn retrieve_state(key_pointer: *const u8, key_length: i32, target_pointer: *const u8) -> i32;
      pub fn db_store_single(key_pointer: *const u8, key_length: i32, value_pointer: *const u8, value_length: i32);
      pub fn db_get_data(key_pointer: *const u8, key_length: i32, target_pointer: *const u8) -> i32;
      pub fn emit(value_pointer: *const u8, value_length: i32);
  }

  pub struct BalanceFn;
  impl JsFn for BalanceFn {
      fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        unsafe {
          let r = balance();
          r.into()
        }
      }
  }

  pub struct TransferFn;
  impl JsFn for TransferFn {
      fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
          if let Some(JsValue::Int(from)) = argv.get(0) {
            if let Some(JsValue::Int(to)) = argv.get(1) {
              if let Some(JsValue::Float(amount)) = argv.get(2) {
                unsafe {
                    let r = transfer(*from, *to, *amount);
                    r.into()
                }
              } else {
                  ctx.throw_type_error("'amount' is not a float").into()
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    use wasmedge_quickjs as q;
    env_logger::init();

    let mut rt = q::Runtime::new();

    let r = rt
        .async_run_with_context(Box::new(|ctx| {

            // add host functions into context
            let f = ctx.new_function::<host_extern::TransferFn>("transfer");
            ctx.get_global().set("transfer", f.into());
            //
            let f = ctx.new_function::<host_extern::BalanceFn>("balance");
            ctx.get_global().set("balance", f.into());

            let (code, mut rest_arg) = args_parse();
            // let code = std::fs::read_to_string(&file_path);
            match code {
                Ok(code) => {
                    rest_arg.insert(0, code.clone());
                    ctx.put_args(rest_arg);
                    ctx.eval_module_str(code, "");
                    // rest_arg.insert(0, file_path.clone());
                    // ctx.put_args(rest_arg);
                    // ctx.eval_buf(code.into_bytes(), &file_path, 1)
                }
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    JsValue::UnDefined
                }
            }
        }))
        .await;
    log::info!("{r:?}");
}


    // rt.run_with_context(|ctx| {

    //     // add host functions into context
    //     let f = ctx.new_function::<host_extern::TransferFn>("transfer");
    //     ctx.get_global().set("transfer", f.into());
    //     //
    //     let f = ctx.new_function::<host_extern::BalanceFn>("balance");
    //     ctx.get_global().set("balance", f.into());
    //     //

    //     let (code, mut rest_arg) = args_parse();
        
    //     rest_arg.insert(0, code.clone());
    //     ctx.put_args(rest_arg);
    //     ctx.eval_module_str(code, "");

    //     ctx.js_loop().unwrap();
    // });

