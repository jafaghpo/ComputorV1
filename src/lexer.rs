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
	Power(u8),
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
			Token::Power(_) => if let Token::Power(_) = other { true } else { false }
			Token::Operator(_) => if let Token::Operator(_) = other { true } else { false }
			Token::Cmp(_) => if let Token::Cmp(_) = other { true } else { false }
		}
	}
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let to_display = match self
		{
			Token::Var((n, d)) => format!("{}X^{}", n, d),
			Token::Power(p) => format!("^{}", p),
			Token::Operator(op) => format!("{}", op.to_string()),
			Token::Cmp(c) => format!("{}", c.to_string())
		};
        write!(f, "{}", to_display)
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
	Ok(Token::Var((number, 0)))
}

pub fn get_var_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
{
	while let Some(next) = chars.peek()
	{
		match next
		{
			'²' => { chars.next(); return Ok(Token::Var(2)) },
			'^' => { chars.next(); break },
			_ if token_list.contains(next) => return Ok(Token::Var(1)),
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
	Err(format!("Syntax error: expected a valid exponent after '^'"))
}

pub fn get_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
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
	Ok(Token::Power(number))
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

pub fn get_tokens(expression: &String) -> Result<(Vec<Token>, Token), String>
{
	let expr = expression.replace(" ", "").replace("\t", "");
	let token_list = get_token_list();
	let mut comparison_token = Token::Cmp(Comparison::No);
	let mut chars = expr.chars().peekable();
	let mut tokens: Vec<Token> = vec![];

	while let Some(c) = chars.next()
	{
		match c
		{
			'0'...'9' => tokens.push(get_number(c, &mut chars, &token_list)?),
			'x' | 'X' => tokens.push(get_var_exponent(&mut chars, &token_list)?),
			'^' => tokens.push(get_exponent(&mut chars, &token_list)?),
			'+' => tokens.push(Token::Operator(Operator::Add)),
			'-' => tokens.push(Token::Operator(Operator::Sub)),
			'*' => tokens.push(Token::Operator(Operator::Mult)),
			'/' => tokens.push(Token::Operator(Operator::Div)),
			'²' => tokens.push(Token::Power(2)),
			'>' | '<' | '=' =>
			{
				if comparison_token != Token::Cmp(Comparison::No)
				{
					return Err(format!("Syntax error: '{}' expression cannot have more than one comparison", c))
				}
				if tokens.len() == 0
				{
					return Err(format!("Syntax error: '{}' cannot be at the beginning of the expression", c))
				}
				let token = get_comparison(c, &mut chars, &token_list)?;
				comparison_token = token.clone();
				tokens.push(token);
			}
			_ => return Err(format!("Lexical error: '{}' is not a valid token", c))
		}
	}
	tokens = reduce_token(tokens)?;
	Ok((tokens, comparison_token))
}

pub fn reduce_token(tokens: Vec<Token>) -> Result<Vec<Token>, String>
{
	let mut new: Vec<Token> = vec![];
	let mut last_token: Option<Token> = None;
	for token in tokens
	{
		if last_token.is_none() { last_token = Some(token) }
		else if let Some(ref last) = last_token
		{
			if last.same_type_as(&token)
			{
				return Err(format!("Syntax error: '{}' is not a valid token after '{}'", token.to_string(), last.to_string()));
			}
			match (&last_token, &token)
			{
				(None, t) => (),
				(Some(Token::Var((n1, d1))), Token::Var((n2, d2))) =>
				{
					if *d1 != 0
					{
						return Err(format!("Syntax error: '{}' is not a valid token after '{}'", token.to_string(), last.to_string()));
					}
					last_token = Some(Token::Var((do_op(*n1, *n2, &Operator::Mult)?, *d2)));
				}
				(Some(Token::Var((n, d))), Token::Power(p)) =>
				{
					if *d != 0
					{
						return Err(format!("Syntax error: '{}' can't be powered up", last_token.unwrap().to_string()));
					}
					last_token = Some(Token::Var((pow(*n, *p), 0)));
				}
				(Some(Token::Operator(Operator::Mult)), Token::Var((n, d))) =>
				{
					let prev_token = new.pop().unwrap();

				}
			}
			println!("last: {:?}", last);
		}
	}
	Ok(new)
}