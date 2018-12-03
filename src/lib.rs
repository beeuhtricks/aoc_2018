extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod day1;
pub mod day2;

aoc_lib!{ year = 2018 }
