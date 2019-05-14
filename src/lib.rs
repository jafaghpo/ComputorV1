use std::fmt;

pub mod parser;
pub mod solver;

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

pub fn format_number(n: f64) -> String
{
	match n.is_finite()
	{
		true => match (n >= 1e6 || n < -1e6, abs(n) < 1e-2 && n != 0.0, n.floor() != n)
		{
			(true, ..) | (_, true, _)=> format!("{:.2e}", n),
			(false, false, true) => format!("{:.2}", n),
            (false, false, false) => format!("{}", n),
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
					2 if *n == 1.0 => format!("x²"),
                    2 => format!("{}x²", format_number(*n)),
					1 if *n == 1.0 => format!("x"),
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

pub fn sqrt(n: f64) -> f64
{
	let mut x0 = n;
	loop
	{
		let x1 = x0 - (x0 * x0 - n) / (2.0 * x0);
		let delta = (x1 - x0) / x0;
		if delta < 0.001 && delta > -0.001 { return x1 }
		x0 = x1;
	}
}

pub fn abs(n: f64) -> f64
{
	if n < 0.0 { n * -1.0 } else { n }
}