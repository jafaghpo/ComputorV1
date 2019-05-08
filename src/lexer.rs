use std::iter::Peekable;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Token
{
	Num(f64),
	Var(u8),
	Power(u8),
	OpenBracket,
	ClosedBracket,
	Greater,
	GreaterEq,
	Lower,
	LowerEq,
	Equal,
	Add,
	Sub,
	Mult,
	Div,
	Empty
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
	Ok(Token::Num(number))
}

pub fn get_var_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
{
	while let Some(next) = chars.peek()
	{
		if *next == ' ' || *next == '\t' { chars.next(); continue }
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
		if *next == ' ' || *next == '\t' { chars.next(); continue }
		let token = match next
		{
			'0' => Ok(Token::Num(1.0)),
			'1' => Ok(Token::Var(1)),
			'2' => Ok(Token::Var(2)),
			'3'...'9' => Err(format!("Syntax error: cannot handle a polynomial degree higher than 2")),
			_ if token_list.contains(next) => Ok(Token::Var(1)),
			_ => Err(format!("Lexical error: '{}' is not a valid token", next))
		};
		chars.next();
		return token;
	}
	Err(format!("Syntax error: expected a valid exponent after '^'"))
}

pub fn get_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>) -> Result<Token, String>
{
	while let Some(next) = chars.peek()
	{
		if *next == ' ' || *next == '\t' { chars.next(); continue }
		let token = match next
		{
			'0' => Ok(Token::Power(0)),
			'1' => Ok(Token::Power(1)),
			'2' => Ok(Token::Power(2)),
			'3'...'9' => Err(format!("Syntax error: cannot handle a power higher than 2")),
			_ => Err(format!("Lexical error: '{}' is not a valid token after '^'", next))
		}?;
		chars.next();
		return Ok(token);
	}
	Err(format!("Syntax error: expected a valid exponent after '^'"))
}

pub fn get_comparison<T: Iterator<Item=char>>(c: char, chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
{
	if let Some(next) = chars.peek()
	{
		if c == '=' && *next != '>' && *next != '<' && *next != '='
		{
			return Ok(Token::Equal);
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
				return Ok(if c == '>' { Token::GreaterEq } else { Token::LowerEq });
			}
			return Ok(if c == '>' { Token::Greater } else { Token::Lower });
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
	let mut list: HashSet<char> = ['+', '-', '/', '*', '^', '²', '>', '<', '=', '(', ')', 'x', 'X', ' ', '\t']
		.iter().cloned().collect();
	let char_num = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
	for n in char_num { list.insert(n); }
	list
}

pub fn get_tokens(expr: &String) -> Result<(Vec<Token>, Token), String>
{
	let token_list = get_token_list();
	let mut comparison_token = Token::Empty;
	let mut chars = expr.chars().peekable();
	let mut tokens: Vec<Token> = vec![];

	while let Some(c) = chars.next()
	{
		match c
		{
			' ' | '\t' => continue,
			'0'...'9' => tokens.push(get_number(c, &mut chars, &token_list)?),
			'x' | 'X' => tokens.push(get_var_exponent(&mut chars, &token_list)?),
			'^' => tokens.push(get_exponent(&mut chars)?),
			'+' => tokens.push(Token::Add),
			'-' => tokens.push(Token::Sub),
			'*' => tokens.push(Token::Mult),
			'/' => tokens.push(Token::Div),
			'²' => tokens.push(Token::Power(2)),
			'(' => tokens.push(Token::OpenBracket),
			')' => tokens.push(Token::ClosedBracket),
			'>' | '<' | '=' =>
			{
				if comparison_token != Token::Empty
				{
					return Err(format!("Syntax error: '{}' expression cannot have more than one comparison", c))
				}
				let token = get_comparison(c, &mut chars, &token_list)?;
				comparison_token = token.clone();
				tokens.push(token);
			}
			_ => return Err(format!("Lexical error: '{}' is not a valid token", c))
		}
	}
	Ok((tokens, comparison_token))
}