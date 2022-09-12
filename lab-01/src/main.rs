use rand::distributions::Alphanumeric;
use rand::distributions::DistString;

use lab_01::damerau_levenshtein;
use lab_01::damerau_levenshtein::DamerauLevenshtein;

/// Замерить среднее время поиска расстояния Дамерау-Левенштейна
fn benchmark<I: DamerauLevenshtein>(len: usize) -> std::time::Duration {

    // Генератор случайных чисел
    let mut rng = rand::thread_rng();

    // Случайные строки данной длины
    let lhs = Alphanumeric.sample_string(&mut rng, len);
    let rhs = Alphanumeric.sample_string(&mut rng, len);

    // Начинаем замер процессорного времени
    let start = cpu_time::ProcessTime::now();

    for _ in 0 .. 1000 {
        <I>::distance(&lhs, &rhs);
    }

    start.elapsed() / 1000
}

fn main() {

    print!("{:<13}\t", "String length");
    print!("{:<13}\t", "Iterative, ns");
    print!("{:<13}\t", "Recursive, ns");
    print!("{:<13}\n", "Memoizing, ns");

    for len in 0 .. 10 {

        print!("{:>13}\t", len);

        print!("{:>13}\t",
            benchmark::<damerau_levenshtein::Iterative>(len).as_nanos());

        print!("{:>13}\t",
            benchmark::<damerau_levenshtein::Recursive>(len).as_nanos());

        print!("{:>13}\n",
            benchmark::<damerau_levenshtein::Memoizing>(len).as_nanos());
    }
}
