
// For a given T = time_limit we can express the result as a function of charging  time x (0..=T):
// movement(t) = 0*x + (T-x) * x.
// Then we're looking for interger solutions to the inequality movement(t) > T
// by the power of Math we observe that the possible solutions are found between the two zero points of the parabola
// so we just need to solve the equality movement(t) - T = 0 and we know how many solutions there are
fn extreme_solutions(time_limit: &i64, record: &i64) -> (f64, f64) {
    let a = -1 as f64;
    let b = *time_limit as f64;
    let c = -1.0 *  (*record as f64);

    let mut sol1 = -b + f64::sqrt(b*b - 4.0 * a* c);
    sol1 /= 2.0 * a;

    let mut sol2 = -b - f64::sqrt(b*b - 4.0 * a* c);
    sol2 /= 2.0 * a;

    return (sol1, sol2);
}

fn  is_integer(x: &f64) -> bool {
    x.fract() == 0.00
}

fn count_solutions((min, max): (f64, f64)) -> i64 {
    let mut narrowing = 0;
    if is_integer(&min) {
        narrowing += 1;
    }
    if is_integer(&max) {
        narrowing += 1;
    }
    let res = (max.floor() + 1.00) - min.ceil();
    return res as i64 - narrowing;
}

pub fn solve(input: &str) -> i64 {
    let times: Vec<i64> = input.lines().next().unwrap().split(':').last().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let distances: Vec<i64> = input.lines().last().unwrap().split(':').last().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let solutions: Vec<_> = times.iter().zip(distances.iter()).map(|(t, d)| extreme_solutions(t, d)).collect();

    let nof_solutions: Vec<_> = solutions.iter().map(|x| count_solutions(*x)).collect();


    return nof_solutions.iter().product();
}