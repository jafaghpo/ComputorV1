use crate::parser::Comparison;

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
			println!("When Y = 0, X = -b / a = {0:.2}\nHence the solution is X = {0:.2}", sol);
		}

		Comparison::Greater =>
		{
			println!("When Y = 0, X = -b / a = {:.2}", sol);
			match positive
			{
				true => println!("Hence the solution is X ∈ ({:.2}, ∞)", sol),
				false =>  println!("Hence the solution is X ∈ (-∞, {:.2})", sol),
			}
		}
		Comparison::GreaterEq =>
		{
			println!("When Y = 0, X = -b / a = {:.2}", sol);
			match positive
			{
				true => println!("Hence the solution is X ∈ [{:.2}, ∞)", sol),
				false =>  println!("Hence the solution is X ∈ (-∞, {:.2}]", sol),
			}
		}
		Comparison::Lower =>
		{
			println!("When Y = 0, X = -b / a = {:.2}", sol);
			match positive
			{
				true => println!("Hence the solution is X ∈ (-∞, {:.2})", sol),
				false =>  println!("Hence the solution is X ∈ ({:.2}, ∞)", sol),
			}
		}
		Comparison::LowerEq =>
		{
			println!("When Y = 0, X = -b / a = {:.2}", sol);
			match positive
			{
				true => println!("Hence the solution is X ∈ (-∞, {:.2}]", sol),
				false =>  println!("Hence the solution is X ∈ [{:.2}, ∞)", sol),
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
		print!("The discriminant is strictly positive");
		sol.push((-coef[1] - delta.sqrt()) / (2.0 * coef[0]));
		sol.push((-coef[1] + delta.sqrt()) / (2.0 * coef[0]));
		println!("When Y = 0, X = {{S1, S2}} with");
		println!("S1 = (-b - √Δ) / 2a = {:.2}", sol[0]);
		println!("S1 = (-b + √Δ) / 2a = {:.2}\n", sol[1]);
		sol.sort_by(|a, b| a.partial_cmp(b).unwrap());
		match cmp_token
		{
			Comparison::Equal =>
			{
				println!("Hence the solution is X = {{{:.2}, {:.2}}}", sol[0], sol[1]);
			}

			Comparison::Greater =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {:.2}) ∪ ({:.2}, ∞)", sol[0], sol[1]),
					false =>  println!("Hence the solution is X ∈ ({:.2} {:.2})", sol[0], sol[1]),
				}
			}
			Comparison::GreaterEq =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {:.2}] ∪ [{:.2}, ∞)", sol[0], sol[1]),
					false =>  println!("Hence the solution is X ∈ [{:.2} {:.2}]", sol[0], sol[1]),
				}
			}
			Comparison::Lower =>
			{
				match positive
				{
					true =>  println!("Hence the solution is X ∈ ({:.2} {:.2})", sol[0], sol[1]),
					false => println!("Hence the solution is X ∈ (-∞, {:.2}) ∪ ({:.2}, ∞)", sol[0], sol[1]),
				}
			}
			Comparison::LowerEq =>
			{
				match positive
				{
					true =>  println!("Hence the solution is X ∈ [{:.2} {:.2}]", sol[0], sol[1]),
					false =>   println!("Hence the solution is X ∈ (-∞, {:.2}] ∪ [{:.2}, ∞)", sol[0], sol[1]),
				}
			}
			_ => ()
		}
	}
	else if delta == 0.0
	{
		print!("The discriminant is null");
		sol.push(-coef[1] / (2.0 * coef[0]));
		println!("When Y = 0, X = -b / 2a = {:.2}", sol[0]);
		match cmp_token
		{
			Comparison::Equal =>
			{
				println!("Hence the solution is X = {:.2}", sol[0]);
			}

			Comparison::Greater =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {0:.2}) ∪ ({0:.2}, ∞)", sol[0]),
					false =>  println!("Hence there is no solution in ℝ"),
				}
			}
			Comparison::GreaterEq =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {0:.2}] ∪ [{0:.2}, ∞)", sol[0]),
					false =>  println!("Hence the solution is X = {:.2}", sol[0]),
				}
			}
			Comparison::Lower =>
			{
				match positive
				{
					true =>  println!("Hence there is no solution in ℝ"),
					false => println!("Hence the solution is X ∈ (-∞, {0:.2}) ∪ ({0:.2}, ∞)", sol[0]),
				}
			}
			Comparison::LowerEq =>
			{
				match positive
				{
					true =>  println!("Hence the solution is X = {:.2}", sol[0]),
					false =>   println!("Hence the solution is X ∈ (-∞, {0:.2}] ∪ [{0:.2}, ∞)", sol[0]),
				}
			}
			_ => ()
		}
	}
	else
	{
		print!("The discriminant is strictly negative");
		sol.push((-coef[1] - abs(delta).sqrt()) / (2.0 * coef[0]));
        sol.push((-coef[1] + abs(delta).sqrt()) / (2.0 * coef[0]));
		println!("When Y = 0, X = {{S1, S2}} with");
		println!("S1 = (-b - √Δ) / 2a = {:.2}i", sol[0]);
		println!("S1 = (-b + √Δ) / 2a = {:.2}i\n", sol[1]);
		sol.sort_by(|a, b| a.partial_cmp(b).unwrap());
		match cmp_token
		{
			Comparison::Equal =>
			{
				println!("Hence there is no solution in ℝ but in ℂ the solution is X = {{{:.2}i, {:.2}i}}", sol[0], sol[1]);
			}

			Comparison::Greater =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {0:.2}) ∪ ({0:.2}, ∞)", sol[0]),
					false =>  println!("Hence there is no solution in ℝ"),
				}
			}
			Comparison::GreaterEq =>
			{
				match positive
				{
					true => println!("Hence the solution is X ∈ (-∞, {0:.2}] ∪ [{0:.2}, ∞)", sol[0]),
					false =>  println!("Hence the solution is X = {:.2}", sol[0]),
				}
			}
			Comparison::Lower =>
			{
				match positive
				{
					true =>  println!("Hence there is no solution in ℝ"),
					false => println!("Hence the solution is X ∈ (-∞, {0:.2}) ∪ ({0:.2}, ∞)", sol[0]),
				}
			}
			Comparison::LowerEq =>
			{
				match positive
				{
					true =>  println!("Hence the solution is X = {:.2}", sol[0]),
					false =>   println!("Hence the solution is X ∈ (-∞, {0:.2}] ∪ [{0:.2}, ∞)", sol[0]),
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
			println!("\na = {:.2}\nb = {:.2}", coef[1], coef[2]);
			solution_degree_1(-coef[2] / coef[1], coef[1] > 0.0, &cmp_token);
		}
		_ =>
		{
			println!("\na = {:.2}\nb = {:.2}\nc = {:.2}", coef[0], coef[1], coef[2]);
			let delta = (coef[1] * coef[1]) - (4.0 * coef[0] * coef[2]);
			println!("Δ = {}\n", delta);
			solution_degree_2(coef, delta, coef[0] > 0.0, &cmp_token);
		}
	}
}




// def get_solution(coef, degree):

//     sol = []
//     if degree == 0:
//         if coef[2] == 0:
//             exit("Every real are solution")
//         exit("No solution")
//     if degree == 1:
//         print("a = {}\nb = {}".format(coef[1], coef[2]))
//         sol.append(-coef[2] / coef[1])
//         print("\nThe solution is:\n-b / a = {:.2f}".format(sol[0]))
//     elif degree == 2:
//         print("a = {}\nb = {}\nc = {}".format(coef[0], coef[1], coef[2]))
//         delta = coef[1] ** 2 - (4 * coef[0] * coef[2])
//         print("Δ = {}".format(delta))
//         if delta > 0:
//             print("\nDiscriminant is strictly positive, the two solutions are:")
//             sol.append((-coef[1] - (delta ** 0.5)) / (2 * coef[0]))
//             sol.append((-coef[1] + (delta ** 0.5)) / (2 * coef[0]))
//             print("(-b - √Δ) / 2a = {:.2f}".format(sol[0]))
//             print("(-b + √Δ) / 2a = {:.2f}".format(sol[1]))
//         else:
//             if delta == 0:
//                 print("Discriminant equals 0, the solution is:")
//                 sol.append(-coef[1] / (2 * coef[0]))
//                 print("-b / 2a = {:.2f}".format(sol[0]))
//             else:
//                 print("Discriminant is strictly negative, the two solutions are:")
//                 sol.append((-coef[1] - (abs(delta) ** 0.5)) / (2 * coef[0]))
//                 sol.append((-coef[1] + (abs(delta) ** 0.5)) / (2 * coef[0]))
//                 print("(-b - √Δ) / 2a = {:.2f}i".format(sol[0]))
//                 print("(-b + √Δ) / 2a = {:.2f}i".format(sol[1]))
                
//     return sol


// def display_graph(coef, sol, degree):

//     x = np.arange(-10.0, 10.0, 0.0001)
//     y = coef[0] * x ** 2 + coef[1] * x + coef[2]
//     plt.plot(x, y)
//     for elem in sol:
//         plt.plot(elem, 0, 'o')
//     plt.title('Equation of degree {}'.format(degree))
//     plt.xlabel('X')
//     plt.ylabel('Y')
//     plt.ylim([-10, 10])
//     plt.grid(True)
//     plt.legend(['Y = {}X^2 + {}X + {}'.format(coef[0], coef[1], coef[2])], loc='best')
//     plt.show()

