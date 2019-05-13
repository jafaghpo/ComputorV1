use std::env;
use std::process::exit;
use std::cmp;

use computor::{parser, solver};
use parser::{Token, Comparison, Operator};
// use lexer::{Token, Comparison};

fn exit_error(msg: &str)
{
	eprintln!("{}", msg);
	exit(1);
}

fn polynomial_degree(coef: &Vec<f64>) -> u8
{
	coef.iter().enumerate().fold(0, |d, (i, n)|
		if *n > 0.0 { cmp::max(2 - i as u8, d) } else { d })
}

fn print_reduced_form(coef: &Vec<f64>, cmp: &Comparison)
{
	let mut i = 2;
	print!("Reduced form:");
	for n in coef
	{
		if *n != 0.0
		{
			print!(" {}", Token::Var((*n, i)));
			match i
			{
				2 => print!(" {}", Token::Operator(Operator::Add)),
				1 if coef[2] != 0.0 => print!(" {}", Token::Operator(Operator::Add)),
				_ => ()
			}
		}
		i -= 1;
	}
	if coef.iter().all(|x| *x == 0.0) { print!(" 0") }

	if *cmp != Comparison::No
	{
		print!(" {} 0", cmp);
	}
	println!("");
}

fn compute_expression(expression: String) -> Result<(), String>
{
	let (tokens, comparison_token) = parser::get_tokens(&expression)?;
	let tokens = parser::reduce_tokens(tokens)?;
	let coef = parser::get_coefficients(&tokens);
	print_reduced_form(&coef, &comparison_token);
	let degree = polynomial_degree(&coef);
	println!("Polynomial degree: {}", degree);
	if comparison_token != Comparison::No
	{
		solver::get_solution(&coef, degree, comparison_token);
	}
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
