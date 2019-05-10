use std::env;
use std::process::exit;

use computor::{lexer, expression, equation};
use lexer::{Token, Comparison};

fn exit_error(msg: &str)
{
	eprintln!("{}", msg);
	exit(1);
}

fn display_result(result: f64)
{
	print!("Result: ");
	match result.is_finite()
	{
		true => match (result >= 1e6, result.floor() != result)
		{
			(true, _) => println!("{:.2e}", result),
			(false, true) => println!("{:.2}", result),
			(false, false) => println!("{}", result),
		}
		false => println!("Infinite number"),
	}
}

fn compute_expression(expression: String) -> Result<(), String>
{
	let (tokens, comparison_token) = lexer::get_tokens(&expression)?;
	if comparison_token == Token::Cmp(Comparison::No)
	{
		let result = expression::parse(&tokens)?;
		display_result(result);
	}
	else
	{
		let coef = equation::parse(&tokens)?;
		println!("coef: {:?}", coef);
	}
	// let coefs = parser::eval_equation(&tokens);
	// println!("Comparison token: {:?}", comparison_token);
	// for token in tokens
	// {
	// 	println!("token: {:?}", token);
	// }
	Ok(())
}

fn main()
{
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 { exit_error("Error: invalid number of arguments") }
	let expr = args[1].to_owned();
	if expr.is_empty() { exit_error("Error: the expression must not be empty") }

	if let Err(e) = compute_expression(expr)
	{
		exit_error(&e);
	}
}
