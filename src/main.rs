use rand::Rng;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{stdin, stdout, Write};
use gettextrs::*;
use std::env::var;

#[derive(Debug, ValueEnum, Clone)]
enum ExerciseType {
    AdditionFindSummand1, // x + 5 = 10
    AdditionFindSummand2, // 5 + x = 10
    AdditionFindSum,      // 5 + 5 = x
    //AdditionFindSumWithinTime,
}
#[derive(Debug, PartialEq)]
enum Exercise {
    Addition(Addition),
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Exercise::Addition(addition) => {
                write!(f, "{}", addition)
            }
        }
    }
}

impl Display for Addition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} + {} = {}",
            number2string(self.s1),
            number2string(self.s2),
            number2string(self.sum)
        )
    }
}

fn number2string(number: Option<Summand>) -> String {
    if let Some(s1) = number {
        s1.to_string()
    } else {
        "_".to_string()
    }
}

enum Sign {
    Plus,
    Minus,
    Times,
    Divides,
    Equals,
}

#[derive(Debug, PartialEq)]
struct Addition {
    s1: Option<Summand>,
    s2: Option<Summand>,
    sum: Option<Sum>,
}

impl Addition {
    fn new(s1: Option<Summand>, s2: Option<Summand>, sum: Option<Sum>) -> Self {
        Self { s1, s2, sum }
    }
}

type Summand = i32;
type Sum = i32;

type Number = i32;

type Solution = i32;

use clap::{Parser, ValueEnum};
use crate::Language::{de_DE, en_US, fr_Fr, Systemlanguage};

#[derive(PartialEq, Debug, ValueEnum, Clone)]
enum Language {
    en_US,
    de_DE,
    fr_Fr,
    Systemlanguage,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::en_US => {
                write!(f, "en_US")
            }
            Language::de_DE => {
                write!(f, "de_DE")
            }
            Language::fr_Fr => {
                write!(f, "fr_FR")
            }
            Language::Systemlanguage => {
                write!(f, "systemlanguage")
            }
        }
    }
}

/// Overengineered Math Learning Game in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of exercises
    #[arg(short, long, default_value_t = 10)]
    count: u32,

    /// Exercise type
    #[arg(short, long)]
    exercise_type: ExerciseType,

    /// Language
    #[arg(short, long, default_value_t = Language::Systemlanguage)]
    language: Language,
}

struct Stats {
    all: u32,
    correct: u32,
}

impl Stats {
    fn new(all: u32) -> Self {
        Stats {
            all,
            correct: 0,
        }
    }
    fn increment_correct(&mut self) {
        self.correct += 1;
    }
    fn get_percent_correct(&self) -> f32{
        (self.correct as f32 / self.all as f32) * 100.0
    }
}

// todo: separate a library crate from the binary crate
fn main() {
    let args = Args::parse();
    let mut counter = 1;
    let count = args.count;
    let exercise_type = args.exercise_type;
    let language = args.language;

    let lang_code = if language == Language::Systemlanguage {
        let temp = std::env::var("LANG").unwrap();
        let (t1, t2) = temp.split_once(".").unwrap();
        t1.to_string()
    }
    else {
        language.clone().to_string()
    };

    println!("{}", lang_code);
    let filename = format!("./locale/{}/myapp.mo", lang_code);
    println!("{}", filename);
    //let f = File::open(filename).expect("could not open the catalog");
    //let catalog = Catalog::parse(f).expect("could not parse the catalog");

    TextDomain::new("myapp")
        .locale(&lang_code)
        .skip_system_data_paths()
        .push("/mnt/Coding/bevy-math-game")
        .codeset("UTF-8") // Optional, the builder does this by default
        .init()
        .unwrap();

    let mut stats = Stats::new(count);

    while counter <= count {
        let (exercise_type, (exercise, solution)) = generate_exercise(
            exercise_type.clone(),
            // todo: move random_integer() to generate_exercise, because it depends on the exercise_type
            // todo: take exercise_type by reference, remove clone() and derive(Clone) from ExerciseType
            // todo: remove exercise_type from return of generate_exercise()
            random_integer(),
            random_integer(),
        );

        //println!("{}", catalog.gettext("Hello World"));
        println!("{}", gettext!("Print Task", counter, count, exercise));
        //print_task(language.clone(), counter, count, exercise);
        let _ = stdout().flush();

        let input = get_and_check_input();

        if solution == input {
            println!("{}", gettext("Print Well Done"));
            stats.increment_correct();
        } else {
            println!("{}", gettext!("Print False", solution));
        }
        counter += 1;
    }
    println!("{}", gettext!("Print Stats", stats.correct, count, format!("{:.2}", stats.get_percent_correct())));
}

fn get_and_check_input() -> i32 {
    loop {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input.truncate(input.len() - 1);
        let input = input.parse::<i32>();
        match input {
            Ok(a) => {
                return a;
            }
            Err(_) => {
                println!("{}", gettext("Print Error: Only numbers"));
                //print_onlynumbers(Args::parse().language.clone());
            }
        }
    }
}

fn random_integer() -> Number {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=10)
}

fn generate_exercise(
    exercise_type: ExerciseType,
    n1: Number,
    n2: Number,
) -> (ExerciseType, (Exercise, Solution)) {
    match exercise_type {
        ExerciseType::AdditionFindSummand1 => {
            (exercise_type, (generate_addition_find_summand1(n1, n2)))
        }
        ExerciseType::AdditionFindSummand2 => {
            (exercise_type, (generate_addition_find_summand2(n1, n2)))
        }
        ExerciseType::AdditionFindSum => (exercise_type, (generate_addition_find_sum(n1, n2))),
    }
}

// _ + n1 = n2
fn generate_addition_find_summand1(n1: Number, n2: Number) -> (Exercise, Solution) {
    let n0 = n1 - n2;
    let exercise = generate_addition(n1, n2);
    let exercise = match exercise {
        Exercise::Addition(a) => Exercise::Addition(Addition::new(None, a.s2, a.sum)),
    };

    if n0 >= 0 {
        (exercise, n0)
    } else {
        (exercise, -n0)
    }
}

fn generate_addition_find_summand2(n1: Number, n2: Number) -> (Exercise, Solution) {
    let n0 = n1 - n2;
    let exercise = generate_addition(n1, n2);
    let exercise = match exercise {
        Exercise::Addition(a) => Exercise::Addition(Addition::new(a.s1, None, a.sum)),
    };

    if n0 >= 0 {
        (exercise, n2)
    } else {
        (exercise, n1)
    }
}

fn generate_addition_find_sum(n1: Number, n2: Number) -> (Exercise, Solution) {
    let n0 = n1 - n2;
    let exercise = generate_addition(n1, n2);
    let exercise = match exercise {
        Exercise::Addition(a) => Exercise::Addition(Addition::new(a.s1, a.s2, None)),
    };

    if n0 >= 0 {
        (exercise, n1)
    } else {
        (exercise, n2)
    }
}

fn generate_addition(n1: Number, n2: Number) -> Exercise {
    let n0 = n1 - n2;
    if n0 >= 0 {
        Exercise::Addition(Addition::new(Some(n0), Some(n2), Some(n1)))
    } else {
        Exercise::Addition(Addition::new(Some(-n0), Some(n1), Some(n2)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_addition_find_summand1() {
        let result = generate_addition_find_summand1(2, 5);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(3), Some(2), Some(5))),
                3
            )
        );

        let result = generate_addition_find_summand1(3, 2);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(1), Some(2), Some(3))),
                1
            )
        );

        let result = generate_addition_find_summand1(4, 4);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(0), Some(4), Some(4))),
                0
            )
        );
    }

    #[test]
    fn test_generate_addition_find_summand2() {
        let result = generate_addition_find_summand2(2, 5);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(3), Some(2), Some(5))),
                2
            )
        );

        let result = generate_addition_find_summand2(3, 2);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(1), Some(2), Some(3))),
                2
            )
        );

        let result = generate_addition_find_summand2(4, 4);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(0), Some(4), Some(4))),
                4
            )
        );
    }

    #[test]
    fn test_generate_addition_find_sum() {
        let result = generate_addition_find_sum(2, 5);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(3), Some(2), Some(5))),
                5
            )
        );

        let result = generate_addition_find_sum(3, 2);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(1), Some(2), Some(3))),
                3
            )
        );

        let result = generate_addition_find_sum(4, 4);
        assert_eq!(
            result,
            (
                Exercise::Addition(Addition::new(Some(0), Some(4), Some(4))),
                4
            )
        );
    }
}