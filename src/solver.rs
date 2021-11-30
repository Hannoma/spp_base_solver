pub struct Outcome<T> {
    pub solution: T,
    pub weight: u64,
}

pub trait Solver<ParsedInput, Solution> {
    fn parse_input() -> ParsedInput;
    fn solve(input: &ParsedInput) -> Outcome<Solution>;
    fn format_solution(solution: &Solution) -> String;
}