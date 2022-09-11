use cpu_time::ProcessTime;

use lab_01::damerau_levenshtein;
use lab_01::damerau_levenshtein::DamerauLevenshtein;

fn main() {

    let start = ProcessTime::now();

    for _ in 0 .. 10000 {
        damerau_levenshtein::Iterative::distance("Hello", "Hola");
    }

    println!("iterative: {:>16?}", start.elapsed());

    let start = ProcessTime::now();

    for _ in 0 .. 10000 {
        damerau_levenshtein::Recursive::distance("Hello", "Hola");
    }

    println!("recursive: {:>16?}", start.elapsed());

    let start = ProcessTime::now();

    for _ in 0 .. 10000 {
        damerau_levenshtein::Memoizing::distance("Hello", "Hola");
    }

    println!("memoizing: {:>16?}", start.elapsed());
}
