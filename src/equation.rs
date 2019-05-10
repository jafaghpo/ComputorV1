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

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<f64>, String>
{
	let mut coef: Vec<f64> = vec![0.0, 0.0, 0.0];
	let mut sign = 1.0;
	let mut last_op = &Add;
	let mut expected: Option<Token> = None;
	let mut iter = tokens.iter().peekable();
	loop
	{
		if let Some(token) = iter.next()
		{
			// println!("token: {:?}", token);
			// println!("coef: {:?}", coef);
			match token
			{
				Operator(Sub) | Num(_) | Var(_) =>
				{
					expected = Some(Operator(Add));
					let tmp_sign = sign;
					let mut nb = if let Num(n) = token { *n } else { 0.0 };
					let mut degree = if let Var(v) = token { Some(*v) } else { None };
					if *token == Operator(Sub) { expected = Some(Num(0.0)) }
					while let Some(next) = iter.peek()
					{
						// println!("next: {:?} expected: {:?}", next, expected);
						match (*next, &expected)
						{
							(Power(p), Some(Operator(_))) =>
							{
								nb = pow(nb, *p);
							}
							(Num(n), Some(Num(_))) =>
							{
								iter.next();
								expected = Some(Operator(Add));
								match iter.peek()
								{
									Some(Power(p)) => { nb = do_op(nb, pow(*n, *p) * tmp_sign, last_op)? }
									_ => { nb = do_op(nb, n * tmp_sign, last_op)?; continue }
								}
							}
							(Var(v), Some(Num(_))) =>
							{
								if degree.is_none() { degree = Some(*v) }
								else { return Err(format!("Syntax error: cannot multiply two variables")) }
								expected = Some(Operator(Add));
							}
							(Operator(Mult), Some(Operator(_))) =>
							{
								last_op = &Mult;
								expected = Some(Num(0.0));
							}
							(Operator(Sub), Some(Num(_))) =>
							{
								sign *= -1.0;
							}
							(_, Some(Operator(_))) =>
							{
								if degree.is_none()
								{
									coef[0] = do_op(coef[0], nb * tmp_sign, last_op)?
								}
								else
								{
									let degree = degree.unwrap() as usize;
									coef[degree] = do_op(coef[degree], nb * tmp_sign, last_op)?;
								}
								break
							}
							_ => return Err(format!("Syntax error: unexpected token '{}'", next.to_string()))
						}
						iter.next();
					}
					if let None = iter.peek()
					{
						if degree.is_none()
						{
							coef[0] = do_op(coef[0], nb * tmp_sign, last_op)?
						}
						else
						{
							let degree = degree.unwrap() as usize;
							coef[degree] = do_op(coef[degree], nb * tmp_sign, last_op)?;
						}
					}
				}
				Cmp(_) if expected == Some(Operator(Add)) => sign = -1.0,
				Operator(op) if expected == Some(Operator(Add)) => last_op = op,
				_ => return Err(format!("Syntax error: unexpected token '{}'", token.to_string()))
			};
		}
		else { break }
	}
	Ok(coef)
}

