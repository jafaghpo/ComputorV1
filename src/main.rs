use std::env;
use std::process::exit;

use computor::{parser, solver};
use computor::{abs};
use computor::{Token, Comparison};

fn exit_error(msg: &str)
{
	eprintln!("{}", msg);
	exit(1);
}

fn polynomial_degree(coef: &Vec<f64>) -> u8
{
	for (i, c) in coef.iter().enumerate()
	{
		if *c != 0.0 { return 2 - i as u8 }
	}
	0
}

fn print_reduced_form(coef: &Vec<f64>, cmp: &Comparison)
{
	// let mut i = 2;

	// for (1n in coef.iter().enumerate()
	// {
	// 	if *n != 0.0
	// 	{
	// 		print!(" {}", Token::Var((*n, i)));
	// 		match i
	// 		{
	// 			2 if coef[1] != 0.0 || coef[2] != 0.0 => print!(" {}", Token::Operator(Operator::Add)),
	// 			1 if coef[2] != 0.0 => print!(" {}", Token::Operator(Operator::Add)),
	// 			_ => ()
	// 		}
	// 	}
	// 	i -= 1;
	// }
	print!("Reduced form:");
	let mut to_display = String::new();
	for (i, n) in coef.iter().enumerate()
	{
		if *n == 0.0 && i != 2 { continue }
		if to_display.len() == 0 { to_display.push_str(&format!(" {}", Token::Var((*n, 2 - i as u8)))); continue }
		match *n >= 0.0
		{
			true => to_display.push_str(&format!(" + {}", Token::Var((*n, 2 - i as u8)))),
			false => to_display.push_str(&format!(" - {}", Token::Var((abs(*n), 2 - i as u8))))
		}
	}
	// if coef.iter().all(|x| *x == 0.0) { to_display.push_str(" 0") }
	print!("{}", to_display);
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
	if comparison_token != Comparison::No
	{
		println!("Polynomial degree: {}", degree);
		solver::get_solution(&coef, degree, comparison_token);
	}
	Ok(())
}

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() != 2
	{
		exit_error("Error: invalid number of arguments");
	}

	let expr = args[1].to_owned();

	if expr.is_empty() { exit_error("Error: the expression must not be empty") }

	if let Err(e) = compute_expression(expr) { exit_error(&e) }
}
