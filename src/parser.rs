use std::iter::Peekable;
use std::collections::HashSet;

use crate::{Token, Comparison, Operator};
use crate::{pow, do_op};

pub fn skip_spaces<T: Iterator<Item=char>>(chars: &mut Peekable<T>)
{
	while let Some(next) = chars.peek()
	{
		if *next != ' ' && *next != '\t' { break }
		chars.next();
	}
}

pub fn get_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<u8, String>
{
	let mut str_number = String::new();
	skip_spaces(chars);
	while let Some(next) = chars.peek()
	{
		match next
		{
			'0'...'9' | '-' => str_number.push(*next),
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
	skip_spaces(chars);
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
	if let Some(c) = chars.peek()
	{
		if *c == 'x' || *c == 'X'
		{
			chars.next();
			if let Token::Var((n, p)) = get_var_exponent(chars, token_list)?
			{
				return Ok(Token::Var((number * n, p)));
			}
		}
	}
	Ok(Token::Var((number, 0)))
}

pub fn get_var_exponent<T: Iterator<Item=char>>(chars: &mut Peekable<T>, token_list: &HashSet<char>) -> Result<Token, String>
{
	skip_spaces(chars);
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
	skip_spaces(chars);
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

// Returns a list of valid tokens
pub fn get_token_list() -> HashSet<char>
{
	let mut list: HashSet<char> = ['+', '-', '/', '*', '^', '²', '>', '<', '=', 'x', 'X', ' ', '\t']
		.iter().cloned().collect();

	// Add all digits in the list of valid tokens
	let char_num = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
	for n in char_num { list.insert(n); }
	list
}

pub fn get_tokens(expression: &String) -> Result<(Vec<Token>, Comparison), String>
{
	// A list of valid tokens
	let token_list = get_token_list();
	
	// A variable holding the type of comparison if there is one
	let mut cmp_token = Comparison::No;
	
	// An iterator on every chars of the expression
	// 'peekable' means that the iterator can check the next char
	let mut chars = expression.chars().peekable();

	// The list of tokens to return
	let mut tokens: Vec<Token> = vec![];

	// Loop through every char of the expression
	while let Some(c) = chars.next()
	{
		match c
		{
			// Ignore char if space
			' ' | '\t' => (),

			// Get the coefficient with its power. ex: 4.5x^2 => Token::Var(4.5, 2)
			'0'...'9' => tokens.push(get_number(c, &mut chars, &token_list)?),

			// Same as above but with coef 1
			'x' | 'X' => tokens.push(get_var_exponent(&mut chars, &token_list)?),

			'^' | '²' => return Err(format!("Syntax error: '^' is only valid after a number or a variable")),
			'+' => tokens.push(Token::Operator(Operator::Add)),
			'-' =>
			{
				skip_spaces(&mut chars);
				// Check the next token to see if the program should treat it as an unary operator or not
				if let Some(next) = chars.peek()
				{
					match next
					{
						// Treat '-' as an unary operator and change the sign of the next number
						'0'...'9' =>
						{
							let num = chars.next().unwrap();
							let token = get_number(num, &mut chars, &token_list)?;
							if let Token::Var((n, p)) = token
							{
								// Check the last stored token to see if the program need to add a '+'
								match tokens.last()
								{
									Some(Token::Operator(_)) | Some(Token::Cmp(_))=> (),
									_ if tokens.len() == 0 => (),
									// If the program is not a operator / cmp and if not the first token,
									// then add a '+' before it
									_ => tokens.push(Token::Operator(Operator::Add))
								}
								// mu
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
				// If there is already a comparison token
				if cmp_token != Comparison::No
				{
					return Err(format!("Syntax error: '{}' expression cannot have more than one comparison", c))
				}
				if tokens.len() == 0
				{
					return Err(format!("Syntax error: '{}' cannot be at the beginning of the expression", c))
				}
				let token = get_comparison(c, &mut chars, &token_list)?;

				// Unwrap the comparison token and store it
				if let Token::Cmp(cmp) = &token { cmp_token = cmp.clone() }

				tokens.push(token);
			}
			_ => return Err(format!("Lexical error: '{}' is not a valid token", c))
		}
	}
	Ok((tokens, cmp_token))
}

// Reduce the coefficients of the expression
pub fn reduce_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, String>
{
	let mut list: Vec<Token> = vec![];
	for current in tokens
	{
		let last = list.pop();

		// Check if there is already a token stored
		match &last
		{
			Some(token) =>
			{
				// There is no valid case with two consecutive tokens of the same type
				// ex: "2 + + 3" | "2 + 3x 5x^2"
				// note that "2 x" is valid
				if token.same_type_as(&current)
				{
					return Err(format!("Syntax error: '{}' is not a valid token after '{}'",
						current, token));
				}
			}
			None =>
			{
				// The first token of the expression should be a variable / coef
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
			// ex: '2' 'X^2' => 2 * X^2
			(Some(Token::Var((n1, d1))), Token::Var((n2, d2))) =>
			{
				// Can't multiply something like '2x^1' '3x^2'
				if *d1 != 0
				{
					return Err(format!("Syntax error: '{}' is not a valid token after '{}'",
						current, last.unwrap()));
				}
				// Multiply the two numbers
				list.push(Token::Var((do_op(*n1, *n2, &Operator::Mult)?, *d2)));
			}
			(Some(Token::Operator(Operator::Mult)), Token::Var((n2, d2))) =>
			{
				let prev = list.pop().unwrap();
				if let Token::Var((n1, d1)) = prev
				{
					// '*' operator can only multiply two numbers with at least one of degree 0
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
					// the denominator cannot be a variable (coef with power greater than 0)
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
				if prev.is_variable() && current.is_variable()
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
	// If the last token of the expression is a coef, then return the list of coefs
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

// Get all the coefs on the same side of the equation and reduce them
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

			// Change the sign of coef on the other side of the equation
			Token::Cmp(_) => sign = -1.0,
			_ => ()
		}
	}
	// ex: 2 + 2x - 3x^2 => - 3x^2 + 2x + 2
	coef.reverse();

	// Return the list of coefs
	coef
}