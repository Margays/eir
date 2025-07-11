use std::sync::LazyLock;

use chrono::DateTime;
use jmespath::{
    Context, ErrorReason, Expression, JmespathError, Rcvar, Runtime, RuntimeError, Variable,
    functions::{ArgumentType, CustomFunction, Signature},
};
use serde_json::Number;

pub static CUSTOM_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    let mut runtime = Runtime::new();
    runtime.register_builtin_functions();
    runtime.register_function(
        "to_timestamp",
        Box::new(CustomFunction::new(
            Signature::new(vec![ArgumentType::String], Some(ArgumentType::Number)),
            Box::new(to_timestamp),
        )),
    );
    runtime
});

#[inline]
pub fn compile(expression: &str) -> Result<Expression<'static>, JmespathError> {
    CUSTOM_RUNTIME.compile(expression)
}

fn to_timestamp(args: &[Rcvar], ctx: &mut Context<'_>) -> Result<Rcvar, JmespathError> {
    if args.len() != 1 {
        return Err(JmespathError::from_ctx(
            ctx,
            ErrorReason::Runtime(RuntimeError::TooManyArguments {
                expected: 1,
                actual: args.len(),
            }),
        ));
    }

    if !args[0].is_string() {
        return Err(JmespathError::from_ctx(
            ctx,
            ErrorReason::Runtime(RuntimeError::InvalidType {
                expected: "string".to_string(),
                actual: args[0].get_type().to_string(),
                position: 0,
            }),
        ));
    }

    let utc_date_str = args[0].as_string().unwrap().replace("UTC", "+00:00");
    let pattern = "%Y-%m-%d %H:%M:%S% %z";
    let date = DateTime::parse_from_str(&utc_date_str, pattern).map_err(|_| {
        JmespathError::from_ctx(
            ctx,
            ErrorReason::Runtime(RuntimeError::InvalidType {
                expected: format!("date with pattern '{pattern}'"),
                actual: args[0].as_string().unwrap().to_string(),
                position: 0,
            }),
        )
    })?;

    let timestamp = date.timestamp() as f64;

    Ok(Rcvar::from(Variable::Number(
        Number::from_f64(timestamp).unwrap(),
    )))
}
