use crate::lexer::{Token, Operator};
use Token::*;
use Operator::*;

pub fn pow(x: f64, n: u8) -> f64
{
	let mut number: f64 = 1.0;
	for _ in 0..n
	{
		number *= x;
	}
	number
}

pub fn do_op(a: f64, b: f64, op: &Operator) -> Result<f64, String>
{
	match op
	{
		Add => Ok(a + b),
		Sub => Ok(a - b),
		Mult => Ok(a * b),
		Div =>
		{
			if b != 0.0 { Ok(a / b) }
			else { Err(format!("Syntax error: attempted to divide {} by zero", a)) }
		}
	}
}

pub fn parse(tokens: &Vec<Token>) -> Result<f64, String>
{
	let mut result: f64 = 0.0;
	let mut expected: Option<Token> = None;
	let mut last_num = 0.0;
	let mut last_op = Add;
	for slice in tokens.chunks(2)
	{
		let mut vec = slice.to_vec();
		let current = Some(vec.remove(0));
		let next = vec.pop();
		match (current, &expected)
		{
			(Some(Operator(Sub)), None) =>
			{
				match next
				{
					Some(Num(n)) =>
					{
						last_op = Sub;
						expected = Some(Operator(Add));
						last_num = n;
					}
					_ => return Err(format!("Syntax error: expected a number at the beginning"))
				}
			}
			(Some(Num(n)), Some(Num(_))) | (Some(Num(n)), None) =>
			{
				match next
				{
					Some(Power(power)) =>
					{
						last_num = pow(n, power);
						expected = Some(Operator(Add));
					}
					Some(Operator(op)) =>
					{
						result = do_op(result, n, &last_op)?;
						last_op = op;
						expected = Some(Num(0.0));
					}
					None =>
					{
						result = do_op(result, n, &last_op)?;
						return Ok(result)
					}
					_ => return Err(format!("Syntax error: expected an operator or a power after {}", n))
				}
			}
			(Some(Operator(op)), Some(Operator(_))) =>
			{
				match next
				{
					Some(Num(n)) =>
					{
						result = do_op(result, last_num, &last_op)?;
						last_num = n;
						last_op = op;
						expected = Some(Operator(Add));
					}
					_ => return Err(format!("Syntax error: expected a number after '{}'", op.to_string()))
				}
			}
			(Some(Power(power)), Some(Operator(_))) =>
			{
				match next
				{
					Some(Operator(op)) =>
					{
						last_num = pow(last_num, power);
						result = do_op(result, last_num, &last_op)?;
						last_op = op;
						expected = Some(Num(0.0));
					}
					Some(Power(p)) =>
					{
						last_num = pow(last_num, power + p);
					}
					None =>
					{
						last_num = pow(last_num, power);
						result = do_op(result, last_num, &last_op)?;
						return Ok(result)
					}
					_ => return Err(format!("Syntax error: expected an operator after a power of {}", power))
				}
			}
			(ref token, Some(t)) =>
			{
				match token
				{
					Some(Num(n)) => return Err(format!("Syntax error: expected {} instead of {}", t.to_string(), n)),
					Some(Operator(op)) => return Err(format!("Syntax error: expected {} instead of '{}'", t.to_string(), op.to_string())),
					Some(Power(p)) => return Err(format!("Syntax error: expected {} instead of a power of {}", t.to_string(), p)),
					Some(Var(_)) => return Err(format!("Syntax error: cannot evaluate an expression with variables")),
					_ => return Err(format!("Syntax error: expected {} instead of nothing", t.to_string())),
				}
			}
			(ref token, None) =>
			{
				match token
				{
					Some(Var(_)) => return Err(format!("Syntax error: cannot evaluate an expression with variables")),
					_ => return Err(format!("Syntax error: expected a number at the beginning"))
				}
			}
		}
	}

	match &expected
	{
		Some(Num(_)) => Err(format!("Syntax error: expected a number at the end")),
		_ =>
		{
			result = do_op(result, last_num, &last_op)?;
			Ok(result)
		}
	}
}

