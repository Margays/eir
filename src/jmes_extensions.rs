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
            Signature::new(
                vec![ArgumentType::String, ArgumentType::String],
                Some(ArgumentType::Number),
            ),
            Box::new(to_timestamp),
        )),
    );
    runtime.register_function(
        "replace",
        Box::new(CustomFunction::new(
            Signature::new(
                vec![
                    ArgumentType::String,
                    ArgumentType::String,
                    ArgumentType::String,
                ],
                Some(ArgumentType::String),
            ),
            Box::new(replace),
        )),
    );
    runtime
});

#[inline]
pub fn compile(expression: &str) -> Result<Expression<'static>, JmespathError> {
    CUSTOM_RUNTIME.compile(expression)
}

fn to_timestamp(args: &[Rcvar], ctx: &mut Context<'_>) -> Result<Rcvar, JmespathError> {
    let utc_date_str = args[0].as_string().unwrap();
    let pattern = args[1].as_string().unwrap();
    let date = DateTime::parse_from_str(utc_date_str, pattern).map_err(|_| {
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

fn replace(args: &[Rcvar], _ctx: &mut Context<'_>) -> Result<Rcvar, JmespathError> {
    let input = args[0].as_string().unwrap();
    let pattern = args[1].as_string().unwrap();
    let replacement = args[2].as_string().unwrap();

    let replaced = input.replace(pattern, replacement);

    Ok(Rcvar::from(Variable::String(replaced)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_timestamp() {
        let expr = "to_timestamp(data, '%Y-%m-%d %H:%M:%S %z')";
        let data = serde_json::json!({"data": "2023-10-01 12:00:00 +0000"});
        let jmes_path = crate::jmes_extensions::compile(expr).unwrap();
        let jmes_value = jmespath::Variable::from_serializable(data).unwrap();
        let value = jmes_path.search(&jmes_value).unwrap();
        assert_eq!(value.as_number().unwrap(), 1696161600.0);
    }

    #[test]
    fn test_replace() {
        let expr = "replace(data, 'foo', 'bar')";
        let data = serde_json::json!({"data": "foo baz foo"});
        let jmes_path = crate::jmes_extensions::compile(expr).unwrap();
        let jmes_value = jmespath::Variable::from_serializable(data).unwrap();
        let value = jmes_path.search(&jmes_value).unwrap();
        assert_eq!(value.as_string().unwrap(), "bar baz bar");
    }
}
