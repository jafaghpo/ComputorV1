use std::env;
use std::process::exit;

use computor::lexer;
// use computor::parser;
use lexer::Token;

fn exit_error(msg: &str)
{
	eprintln!("{}", msg);
	exit(1);
}

fn main()
{
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 { exit_error("Error: invalid number of arguments") }
	let expr = args[1].to_owned();
	if expr.is_empty() { exit_error("Error: the expression must not be empty") }
	let (tokens, comparison_token) = match lexer::get_tokens(&expr)
	{
		Ok(t) => t,
		Err(e) => { exit_error(&e); (vec![], Token::Empty) }
	};
	// if comparison_token == Token::Empty
	// {
	// 	let result = parser::eval_expression(&tokens);
	// 	println!("{}", result);
	// 	return
	// }
	// let coefs = parser::eval_equation(&tokens);
	println!("Comparison token: {:?}", comparison_token);
	for token in tokens
	{
		println!("token: {:?}", token);
	}
}
