use self::float_duration::FloatDuration;
use crate::{builtins::*, class::Class};
use float_duration;
use jazz_vm::{function::Function, machine::Machine, value::Value};
use std::{cell::UnsafeCell, collections::HashMap, time::Instant};

pub fn time(m: &mut Machine, _: Vec<Value>) -> Value
{
    let now = Instant::now();
    let duration = FloatDuration::from_std(now.elapsed());

    let str = format!("{}", duration);
    let obj = Value::Object(m.pool.allocate(Box::new(str)));
    return obj;
}

pub fn system_class(m: &mut Machine) -> Class
{
    let mut fields = HashMap::new();
    let f = Function::from_native(Box::new(print));
    fields.insert(
        "print".to_owned(),
        Value::Object(m.pool.allocate(Box::new(f))),
    );
    fields.insert(
        "readln".to_owned(),
        Value::Object(
            m.pool
                .allocate(Box::new(Function::from_native(Box::new(readln)))),
        ),
    );
    fields.insert(
        "time".to_owned(),
        Value::Object(
            m.pool
                .allocate(Box::new(Function::from_native(Box::new(time)))),
        ),
    );
    Class {
        name: String::from("System"),
        fields: UnsafeCell::new(fields),
    }
}

pub fn unary_minus(m: &mut Machine) -> Value
{
    use jazz_vm::opcodes::Instruction::*;

    let code = vec![LoadLong(2, 0), Move(3, 1), Sub(3, 2, 3), Ret(3)];

    let func = Function::from_instructions(code, 1);
    Value::Object(m.pool.allocate(Box::new(func)))
}
