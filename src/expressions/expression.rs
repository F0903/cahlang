use super::sub_expressions::SubExpression;
use crate::identifiable::Identifiable;
use crate::operators::{self, Operation};
use crate::value::Value;
use crate::vm::ExecutionContext;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Expression<'a> {
    expr_string: String,
    context: &'a dyn ExecutionContext,
}

impl<'a> Expression<'a> {
    pub fn from_str(expr: impl ToString, context: &'a dyn ExecutionContext) -> Expression {
        Expression {
            expr_string: expr.to_string(),
            context,
        }
    }

    fn get_value_from_expr_token(&self, expr_token: &str) -> Result<Value> {
        let var = self.context.get_var(expr_token);
        let value = match var {
            Some(x) => x.borrow().get_value(),
            None => Value::from_string(expr_token)?,
        };
        Ok(value)
    }

    fn is_char_operator(ch: char) -> Option<&'a Operation> {
        for op in operators::OPERATORS {
            if op.get_identifier().chars().all(|x| x == ch) {
                return Some(op);
            }
        }
        None
    }

    /// NOTE: Expects no spacing in input
    fn parse_expr(self) -> Result<Value> {
        let expression_str = &self.expr_string;
        let mut token_values = vec![];
        let mut ops = vec![];
        let mut token_buf = String::default();

        for ch in expression_str.chars() {
            if let Some(op) = Self::is_char_operator(ch) {
                let value = self.get_value_from_expr_token(&token_buf)?;
                token_values.push(value);
                token_buf.clear();
                ops.push(Some(op));
                continue;
            }
            token_buf.push(ch);
        }

        if !token_buf.is_empty() {
            let value = self.get_value_from_expr_token(&token_buf)?;
            token_values.push(value);
            token_buf.clear();
        }

        if ops.is_empty() {
            if token_values.len() > 1 {
                return Err(
                    "No operators was found in expression, but more than one token was found!"
                        .into(),
                );
            }
            return Ok(token_values[0].clone());
        }

        ops.push(None);
        ops.reverse();
        let ops_iter = ops.iter();

        token_values.reverse();
        let value_iter = token_values.iter();

        let value_op_iter = value_iter.zip(ops_iter);

        let mut current_sub_expr;
        let mut next_sub_expr: Option<SubExpression> = None;
        for (value, op) in value_op_iter {
            current_sub_expr = SubExpression {
                value: value.clone(),
                op: match op {
                    None => operators::NOOP,
                    Some(x) => (*x).clone(),
                },
                next: next_sub_expr.map(Box::new),
            };
            next_sub_expr = Some(current_sub_expr);
        }

        let mut start_node = match next_sub_expr {
            None => return Err("No start node found!".into()),
            Some(x) => x,
        };
        let val = start_node.evaluate()?;

        Ok(val)
    }

    pub fn evaluate(self) -> Result<Value> {
        self.parse_expr()
    }
}
