use std::iter::Peekable;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Comparison
{
	Equal,
	Greater,
	GreaterEq,
	Lower,
	LowerEq,
	No
}

impl fmt::Display for Comparison
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let to_display = match self
		{
			Comparison::Equal => "=",
			Comparison::Greater => ">",
			Comparison::GreaterEq => ">=",
			Comparison::Lower => "<",
			Comparison::LowerEq => "<=",
			Comparison::No => "nothing"
		};
        write!(f, "{}", to_display)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator
{
	Add,
	Sub,
	Div,
	Mult
}

impl fmt::Display for Operator
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let to_display = match self
		{
			Operator::Add => "+",
			Operator::Sub => "-",
			Operator::Div => "/",
			Operator::Mult => "*"
		};
        write!(f, "{}", to_display)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token
{
	Var((f64, u8)),
	Operator(Operator),
	Cmp(Comparison),
}

impl Token
{
	pub fn same_type_as(&self, other: &Self) -> bool
	{
		match self
		{
			Token::Var((_, 0)) => if let Token::Var((_, 0)) = other { true } else { false }
			Token::Var(_) => if let Token::Var(_) = other { true } else { false }
			Token::Operator(_) => if let Token::Operator(_) = other { true } else { false }
			Token::Cmp(_) => if let Token::Cmp(_) = other { true } else { false }
		}
	}

	pub fn is_variable(&self) -> bool
	{
		match self
		{
			Token::Var(_) => true,
			_ => false
		}
	}
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let to_display = match self
		{
			Token::Var((n, d)) =>
			{
				match d
				{
					2 => format!("{}x²", format_number(*n)),
					1 => format!("{}x", format_number(*n)),
					0 | _ => format!("{}", format_number(*n))
				}
			}
			Token::Operator(op) => format!("{}", op),
			Token::Cmp(c) => format!("{}", c)
		};
        write!(f, "{}", to_display)
    }
}

pub fn format_number(n: f64) -> String
{
	match n.is_finite()
	{
		true => match (n >= 1e6 || n <= -1e6, n.floor() != n)
		{
			(true, _) => format!("{:.2e}", n),
			(false, true) => format!("{:.2}", n),
			(false, false) => format!("{}", n),
		}
		false =>
		{
			match n.is_sign_positive()
			{
				true => format!("∞"),
				false => format!("-∞")
			}
		}
	}
}

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
		Operator::Add => Ok(a + b),
		Operator::Sub => Ok(a - b),
		Operator::Mult => Ok(a * b),
		Operator::Div =>
		{
			if b != 0.0 { Ok(a / b) }
			else { Err(format!("Syntax error: attempted to divide {} by zero", a)) }
		}
	}
}

pub fn get_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<u8, String>
{
	let mut str_number = String::new();
	while let Some(next) = chars.peek()
	{
		match next
		{
			'0'...'9' => str_number.push(*next),
			_ if !token_list.contains(next) =>
			{
				return Err(format!("Lexical error: '{}' is not a valid token", next))
			}
			_ => break
		};
		chars.next();
	}
	let number = match str_number.parse::<u8>()
	{
		Ok(n) => Ok(n),
		Err(_) => Err(format!("Syntax error: '{}' is not a valid exponent", str_number))
	}?;
	Ok(number)
}

pub fn get_number<T: Iterator<Item=char>>(c: char, chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
{
    let mut str_number = String::new();
	str_number.push(c);
	while let Some(next) = chars.peek()
	{
		match next
		{
			'0'...'9' | '.' => str_number.push(*next),
			_ if !token_list.contains(next) =>
			{
				return Err(format!("Lexical error: '{}' is not a valid token", next))
			}
			_ => break
		};
		chars.next();
	}
	if let Some('.') = str_number.chars().last()
	{
		return Err(format!("Syntax error: '{}' is not a valid number", str_number));
	}
	let number = match str_number.parse::<f64>()
	{
		Ok(n) => Ok(n),
		Err(_) => Err(format!("Syntax error: '{}' is not a valid number", str_number))
	}?;
	if let Some('^') = chars.peek()
	{
		chars.next();
		let power = get_exponent(chars, token_list)?;
		return Ok(Token::Var((pow(number, power), 0)));
	}
	if let Some('²') = chars.peek()
	{
		chars.next();
		return Ok(Token::Var((pow(number, 2), 0)));
	}
	Ok(Token::Var((number, 0)))
}

pub fn get_var_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
{
	while let Some(next) = chars.peek()
	{
		match next
		{
			'²' => { chars.next(); return Ok(Token::Var((1.0, 2))) },
			'^' => { chars.next(); break },
			_ if token_list.contains(next) => return Ok(Token::Var((1.0, 1))),
			_ => return Err(format!("Lexical error: '{}' is not a valid token", next))
		}
	}
	while let Some(next) = chars.peek()
	{
		let token = match next
		{
			'0' => Ok(Token::Var((1.0, 0))),
			'1' => Ok(Token::Var((1.0, 1))),
			'2' => Ok(Token::Var((1.0, 2))),
			'3'...'9' => Err(format!("Syntax error: cannot handle a polynomial degree higher than 2")),
			_ if token_list.contains(next) => Ok(Token::Var((1.0, 1))),
			_ => Err(format!("Lexical error: '{}' is not a valid token", next))
		};
		chars.next();
		return token;
	}
	match chars.peek()
	{
		None => Ok(Token::Var((1.0, 1))),
		_ => Err(format!("Syntax error: expected a valid exponent after '^'"))
	}
}

pub fn get_comparison<T: Iterator<Item=char>>(c: char, chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
{
	if let Some(next) = chars.peek()
	{
		if c == '=' && *next != '>' && *next != '<' && *next != '='
		{
			return Ok(Token::Cmp(Comparison::Equal));
		}
		else if (c == '>' || c == '<') && *next != '>' && *next != '<'
		{
			if token_list.contains(next) == false
			{
				return Err(format!("Lexical error: '{}' is not a valid token", next));
			}
			if *next == '='
			{
				chars.next();
				return Ok(if c == '>' { Token::Cmp(Comparison::GreaterEq) } else { Token::Cmp(Comparison::LowerEq) });
			}
			return Ok(if c == '>' { Token::Cmp(Comparison::Greater) } else { Token::Cmp(Comparison::Lower) });
		}
		else
		{
			return Err(format!("Syntax error: '{}' is invalid after '{}'", c, next));
		}
	}
	Err(format!("Syntax error: '{}' cannot be at the end of the expression", c))
}

pub fn get_token_list() -> HashSet<char>
{
	let mut list: HashSet<char> = ['+', '-', '/', '*', '^', '²', '>', '<', '=', 'x', 'X', ' ', '\t']
		.iter().cloned().collect();
	let char_num = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
	for n in char_num { list.insert(n); }
	list
}

pub fn get_tokens(expression: &String) -> Result<(Vec<Token>, Comparison), String>
{
	let expr = expression.replace(" ", "").replace("\t", "");
	let token_list = get_token_list();
	let mut cmp_token = Comparison::No;
	let mut chars = expr.chars().peekable();
	let mut tokens: Vec<Token> = vec![];

	while let Some(c) = chars.next()
	{
		match c
		{
			'0'...'9' => tokens.push(get_number(c, &mut chars, &token_list)?),
			'x' | 'X' => tokens.push(get_var_exponent(&mut chars, &token_list)?),
			'^' | '²' => return Err(format!("Syntax error: '^' is only valid after a number or a variable")),
			'+' => tokens.push(Token::Operator(Operator::Add)),
			'-' =>
			{
				if let Some(next) = chars.peek()
				{
					match next
					{
						'0'...'9' =>
						{
							let num = chars.next().unwrap();
							let token = get_number(num, &mut chars, &token_list)?;
							if let Token::Var((n, p)) = token
							{
								tokens.push(Token::Operator(Operator::Add));
								tokens.push(Token::Var((n * -1.0, p)));
							}
						}
						_ => tokens.push(Token::Operator(Operator::Sub))
					};
					continue
				}
				return Err(format!("Syntax error: '-' is invalid at the end"));
			}
			'*' => tokens.push(Token::Operator(Operator::Mult)),
			'/' => tokens.push(Token::Operator(Operator::Div)),
			'>' | '<' | '=' =>
			{
				if cmp_token != Comparison::No
				{
					return Err(format!("Syntax error: '{}' expression cannot have more than one comparison", c))
				}
				if tokens.len() == 0
				{
					return Err(format!("Syntax error: '{}' cannot be at the beginning of the expression", c))
				}
				let token = get_comparison(c, &mut chars, &token_list)?;
				if let Token::Cmp(cmp) = &token { cmp_token = cmp.clone() }
				tokens.push(token);
			}
			_ => return Err(format!("Lexical error: '{}' is not a valid token", c))
		}
	}
	Ok((tokens, cmp_token))
}

pub fn reduce_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, String>
{
	let mut list: Vec<Token> = vec![];
	for current in tokens
	{
		let last = list.pop();
		match &last
		{
			Some(token) =>
			{
				if token.same_type_as(&current)
				{
					return Err(format!("Syntax error: '{}' is not a valid token after '{}'",
						current, token));
				}
			}
			None =>
			{
				if current.is_variable() == false
				{
					return Err(format!("Syntax error: expected a variable or number instead of '{}'",
						current));
				}
				list.push(current);
				continue
			}
		}

		match (&last, &current)
		{
			(Some(Token::Var((n1, d1))), Token::Var((n2, d2))) =>
			{
				if *d1 != 0
				{
					return Err(format!("Syntax error: '{}' is not a valid token after '{}'",
						current, last.unwrap()));
				}
				list.push(Token::Var((do_op(*n1, *n2, &Operator::Mult)?, *d2)));
			}
			(Some(Token::Operator(Operator::Mult)), Token::Var((n2, d2))) =>
			{
				let prev = list.pop().unwrap();
				if let Token::Var((n1, d1)) = prev
				{
					if d1 > 0 && *d2 > 0
					{
						return Err(format!("Syntax error: can't multiply '{}' with '{}'",
							prev, current));
					}
					list.push(Token::Var((do_op(n1, *n2, &Operator::Mult)?, d1 + d2)));
				}
			}
			(Some(Token::Operator(Operator::Div)), Token::Var((n2, d2))) =>
			{
				let prev = list.pop().unwrap();
				if let Token::Var((n1, d1)) = prev
				{
					if *d2 > 0
					{
						return Err(format!("Syntax error: can't divide '{}' with '{}'",
							prev, current));
					}
					list.push(Token::Var((do_op(n1, *n2, &Operator::Div)?, d1)));
				}
			}
			(Some(Token::Cmp(_)), _) =>
			{
				let prev = list.pop().unwrap();
				if prev.is_variable() && prev.same_type_as(&current)
				{
					list.push(prev);
					list.push(last.unwrap());
					list.push(current);
				}
				else
				{
					return Err(format!("Syntax error: '{}' need to be between two numbers or variables",
						last.unwrap()));
				}
			}
			_ =>
			{
				list.push(last.unwrap());
				list.push(current);
			}
		}
	}
	if let Some(Token::Var(_)) = list.last()
	{
		Ok(list)
	}
	else
	{
		return Err(format!("Syntax error: expected a variable or number instead of '{}'",
			list.pop().unwrap()));
	}
}

pub fn get_coefficients(tokens: &Vec<Token>) -> Vec<f64>
{
	let mut coef: Vec<f64> = vec![0.0, 0.0, 0.0];
	let mut sign = 1.0;
	let mut op = Operator::Add;
	for token in tokens
	{
		match token
		{
			Token::Var((n, d)) =>
			{
				let degree = *d as usize;
				coef[degree] = do_op(coef[degree], n * sign, &op).unwrap();
			}
			Token::Operator(Operator::Add) => op = Operator::Add,
			Token::Operator(Operator::Sub) => op = Operator::Sub,
			Token::Cmp(_) => sign = -1.0,
			_ => ()
		}
	}
	coef.reverse();
	coef
}