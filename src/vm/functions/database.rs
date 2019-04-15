use vm::types::{Value};
use vm::representations::{SymbolicExpression, SymbolicExpressionType};
use vm::errors::{Error, ErrType, InterpreterResult as Result};
use vm::{eval, LocalContext, Environment};

pub fn special_contract_call(args: &[SymbolicExpression],
                         env: &mut Environment,
                         context: &LocalContext) -> Result<Value> {
    if args.len() < 2 {
        return Err(Error::new(ErrType::InvalidArguments(
            "(contract-call ...) requires at least 2 arguments: the contract name and the public function name".to_string())))
    }

    let contract_name = match &args[0].expr {
        SymbolicExpressionType::Atom(value) => Ok(value),
        _ => Err(Error::new(ErrType::InvalidArguments("First argument to (contract-call ...) must be contract name".to_string())))
    }?;

    let function_name = match &args[1].expr {
        SymbolicExpressionType::Atom(value) => Ok(value),
        _ => Err(Error::new(ErrType::InvalidArguments("Second argument to (contract-call ...) must be function name".to_string())))
    }?;

    let rest_args = &args[2..];

    let rest_args: Result<Vec<_>> = rest_args.iter().map(|x| { eval(x, env, context) }).collect();
    let mut rest_args = rest_args?;
    let rest_args: Vec<_> = rest_args.drain(..).map(|x| { SymbolicExpression::atom_value(x) }).collect();

    if env.sender.is_none() {
        return Err(Error::new(ErrType::InvalidArguments(
            "No sender in current context. Did you attempt to (contract-call ...) from a non-contract aware environment?"
                .to_string())));
    }

    env.execute_contract(
        contract_name, function_name, &rest_args)
        .map_err(|mut x| {
            if x.has_stack_trace() {
                x.extend_with(env.call_stack.make_stack_trace())
            }
            x
        })
}

pub fn special_fetch_entry(args: &[SymbolicExpression],
                           env: &mut Environment,
                           context: &LocalContext) -> Result<Value> {
    // arg0 -> map name
    // arg1 -> key
    if args.len() != 2 {
        return Err(Error::new(ErrType::InvalidArguments("(fetch-entry ...) requires exactly 2 arguments".to_string())))
    }

    let key = eval(&args[1], env, context)?;

    let map_name = match &args[0].expr {
        SymbolicExpressionType::Atom(value) => Ok(value),
        _ => Err(Error::new(ErrType::InvalidArguments("First argument in data functions must be the map name".to_string())))
    }?;

    env.global_context.database.fetch_entry(&env.contract_context.name, &map_name, &key)
}

pub fn special_set_entry(args: &[SymbolicExpression],
                         env: &mut Environment,
                         context: &LocalContext) -> Result<Value> {
    // arg0 -> map name
    // arg1 -> key
    // arg2 -> value
    if args.len() != 3 {
        return Err(Error::new(ErrType::InvalidArguments("(set-entry! ...) requires exactly 3 arguments".to_string())))
    }

    let key = eval(&args[1], env, context)?;
    let value = eval(&args[2], env, context)?;

    let map_name = match &args[0].expr {
        SymbolicExpressionType::Atom(value) => Ok(value),
        _ => Err(Error::new(ErrType::InvalidArguments("First argument in data functions must be the map name".to_string())))
    }?;

    env.global_context.database.set_entry(&env.contract_context.name, &map_name, key, value)
}

pub fn special_insert_entry(args: &[SymbolicExpression],
                            env: &mut Environment,
                            context: &LocalContext) -> Result<Value> {
    // arg0 -> map name
    // arg1 -> key
    // arg2 -> value
    if args.len() != 3 {
        return Err(Error::new(ErrType::InvalidArguments("(insert-entry! ...) requires exactly 3 arguments".to_string())))
    }

    let key = eval(&args[1], env, context)?;
    let value = eval(&args[2], env, context)?;

    let map_name = match &args[0].expr {
        SymbolicExpressionType::Atom(value) => Ok(value),
        _ => Err(Error::new(ErrType::InvalidArguments("First argument in data functions must be the map name".to_string())))
    }?;

    env.global_context.database.insert_entry(&env.contract_context.name, &map_name, key, value)
}

pub fn special_delete_entry(args: &[SymbolicExpression],
                            env: &mut Environment,
                            context: &LocalContext) -> Result<Value> {
    // arg0 -> map name
    // arg1 -> key
    if args.len() != 2 {
        return Err(Error::new(ErrType::InvalidArguments("(delete-entry! ...) requires exactly 2 arguments".to_string())))
    }

    let key = eval(&args[1], env, context)?;

    let map_name = match &args[0].expr {
        SymbolicExpressionType::Atom(value) => Ok(value),
        _ => Err(Error::new(ErrType::InvalidArguments("First argument in data functions must be the map name".to_string())))
    }?;

    env.global_context.database.delete_entry(&env.contract_context.name, &map_name, &key)
}
