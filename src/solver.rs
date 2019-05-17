use crate::Comparison;
use crate::{abs, format_number};

fn solution_degree_0(sol: f64, cmp_token: &Comparison)
{
	match cmp_token
	{
		Comparison::Equal if sol == 0.0		=> println!("The solution is X ∈ ℝ"),
		Comparison::Greater if sol > 0.0	=> println!("The solution is X ∈ ℝ"),
		Comparison::GreaterEq if sol >= 0.0	=> println!("The solution is X ∈ ℝ"),
		Comparison::Lower if sol < 0.0		=> println!("The solution is X ∈ ℝ"),
		Comparison::LowerEq if sol <= 0.0	=> println!("The solution is X ∈ ℝ"),
		_ => println!("There is no solution in ℝ")
	}
}

fn solution_degree_1(sol: f64, positive: bool, cmp_token: &Comparison)
{
	match cmp_token
	{
		Comparison::Equal =>
		{
			println!("When Y = 0, X = -b / a = {0}\nHence the solution is X = {0}", format_number(sol));
		}

		Comparison::Greater =>
		{
			println!("When Y = 0, X = -b / a = {0}", format_number(sol));
			match positive
			{
				true => println!("Hence the solution is X ∈ ({0}, ∞)", format_number(sol)),
				false =>  println!("Hence the solution is X ∈ (-∞, {0})", format_number(sol)),
			}
		}
		Comparison::GreaterEq =>
		{
			println!("When Y = 0, X = -b / a = {0}", format_number(sol));
			match positive
			{
				true => println!("Hence the solution is X ∈ [{0}, ∞)", format_number(sol)),
				false =>  println!("Hence the solution is X ∈ (-∞, {0}]", format_number(sol)),
			}
		}
		Comparison::Lower =>
		{
			println!("When Y = 0, X = -b / a = {0}", format_number(sol));
			match positive
			{
				true => println!("Hence the solution is X ∈ (-∞, {0})", format_number(sol)),
				false =>  println!("Hence the solution is X ∈ ({0}, ∞)", format_number(sol)),
			}
		}
		Comparison::LowerEq =>
		{
			println!("When Y = 0, X = -b / a = {0}", format_number(sol));
			match positive
			{
				true => println!("Hence the solution is X ∈ (-∞, {0}]", format_number(sol)),
				false =>  println!("Hence the solution is X ∈ [{0}, ∞)", format_number(sol)),
			}
		}
		_ => ()
	}
}

fn solution_degree_2(coef: &Vec<f64>, delta: f64, positive: bool, cmp_token: &Comparison)
{
	let mut sol: Vec<f64> = vec![];
	if delta > 0.0
	{
		println!("The discriminant is strictly positive");
		sol.push((-coef[1] - delta.sqrt()) / (2.0 * coef[0]));
		sol.push((-coef[1] + delta.sqrt()) / (2.0 * coef[0]));
		println!("When Y = 0, X = {{S1, S2}} with");
		println!("S1 = (-b - √Δ) / 2a = {}", format_number(sol[0]));
		println!("S2 = (-b + √Δ) / 2a = {}\n", format_number(sol[1]));
		sol.sort_by(|a, b| a.partial_cmp(b).unwrap());
		let sol = vec![format_number(sol[0]), format_number(sol[1])];
		match cmp_token
		{
			Comparison::Equal =>
			{
				println!("Hence the solution is X = {{{}, {}}}", sol[0], sol[1]);
			}

			Comparison::Greater =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {}) ∪ ({}, ∞)", sol[0], sol[1]),
					false =>  println!("Hence the solution is X ∈ ({}, {})", sol[0], sol[1]),
				}
			}
			Comparison::GreaterEq =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {}] ∪ [{}, ∞)", sol[0], sol[1]),
					false =>  println!("Hence the solution is X ∈ [{}, {}]", sol[0], sol[1]),
				}
			}
			Comparison::Lower =>
			{
				match positive
				{
					true =>  println!("Hence the solution is X ∈ ({}, {})", sol[0], sol[1]),
					false => println!("Hence the solution is X ∈ (-∞, {}) ∪ ({}, ∞)", sol[0], sol[1]),
				}
			}
			Comparison::LowerEq =>
			{
				match positive
				{
					true =>  println!("Hence the solution is X ∈ [{}, {}]", sol[0], sol[1]),
					false =>   println!("Hence the solution is X ∈ (-∞, {}] ∪ [{}, ∞)", sol[0], sol[1]),
				}
			}
			_ => ()
		}
	}
	else if delta == 0.0
	{
		println!("The discriminant is null");
		sol.push(-coef[1] / (2.0 * coef[0]));
		let sol = format_number(sol[0]);
		println!("When Y = 0, X = -b / 2a = {}", sol);
		match cmp_token
		{
			Comparison::Equal =>
			{
				println!("Hence the solution is X = {}", sol);
			}

			Comparison::Greater =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {0}) ∪ ({0}, ∞)", sol),
					false =>  println!("Hence there is no solution in ℝ"),
				}
			}
			Comparison::GreaterEq =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {0}] ∪ [{0}, ∞)", sol),
					false =>  println!("Hence the solution is X = {}", sol),
				}
			}
			Comparison::Lower =>
			{
				match positive
				{
					true =>  println!("Hence there is no solution in ℝ"),
					false => println!("Hence the solution is X ∈ (-∞, {0}) ∪ ({0}, ∞)", sol),
				}
			}
			Comparison::LowerEq =>
			{
				match positive
				{
					true =>  println!("Hence the solution is X = {}", sol),
					false =>   println!("Hence the solution is X ∈ (-∞, {0}] ∪ [{0}, ∞)", sol),
				}
			}
			_ => ()
		}
	}
	else
	{
		println!("The discriminant is strictly negative");
		sol.push((-coef[1] - abs(delta).sqrt()) / (2.0 * coef[0]));
        sol.push((-coef[1] + abs(delta).sqrt()) / (2.0 * coef[0]));
		println!("When Y = 0, X = {{S1, S2}} with");
		println!("S1 = (-b - √Δ) / 2a = {}i", format_number(sol[0]));
		println!("S2 = (-b + √Δ) / 2a = {}i\n", format_number(sol[1]));
		sol.sort_by(|a, b| a.partial_cmp(b).unwrap());
		let sol = vec![format_number(sol[0]), format_number(sol[1])];
		match cmp_token
		{
			Comparison::Equal =>
			{
				println!("In ℝ, there is no solution");
				println!("In ℂ, the solution is X = {{{}i, {}i}}", sol[0], sol[1]);
			}

			Comparison::Greater | Comparison::GreaterEq =>
			{
				match positive
				{
					true => println!("The solution is X ∈ ℝ"),
					false =>  println!("There is no solution in ℝ")
				}
			}
			Comparison::Lower | Comparison::LowerEq =>
			{
				match positive
				{
					true =>  println!("There is no solution in ℝ"),
					false => println!("The solution is X ∈ ℝ"),
				}
			}
			_ => ()
		}
	}
}

pub fn get_solution(coef: &Vec<f64>, degree: u8, cmp_token: Comparison)
{
	if coef.iter().any(|x| x.is_infinite())
	{
		println!("The solution can't be found because one of the terms is infinite");
		return
	}
	match degree
	{
		0 => solution_degree_0(coef[2], &cmp_token),
		1 =>
		{
			println!("\na = {}\nb = {}\n", format_number(coef[1]), format_number(coef[2]));
			solution_degree_1(-coef[2] / coef[1], coef[1] > 0.0, &cmp_token);
		}
		_ =>
		{
			println!("\na = {}\nb = {}\nc = {}", format_number(coef[0]), format_number(coef[1]), format_number(coef[2]));
			let delta = (coef[1] * coef[1]) - (4.0 * coef[0] * coef[2]);
			println!("Δ = {}\n", format_number(delta));
			solution_degree_2(coef, delta, coef[0] > 0.0, &cmp_token);
		}
	}
}
