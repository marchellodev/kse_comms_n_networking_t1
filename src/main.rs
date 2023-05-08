use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::sync::Arc;
use std::{thread, time::Instant};

const RNG_FROM: i32 = 1_000;
const RNG_TO: i32 = 9_999;
const RNG_SEED: [u8; 32] = [1; 32];

const MATRIX_SIZE: usize = 50_000;
const THREADS: usize = 128;
const PRINT_MATRICES: bool = false;

type Matrix = Vec<Vec<i32>>;

fn main() {
    let now = Instant::now();

    println!("> program init");

    // let mut rng = rand::thread_rng();
    let mut rng = SmallRng::from_seed(RNG_SEED);

    let a = Arc::new(generate_matrix(&mut rng));
    let b = Arc::new(generate_matrix(&mut rng));

    if PRINT_MATRICES {
        print_matrix(&a);
        println!();
        print_matrix(&b);
    }

    let setup_elapsed = now.elapsed();
    println!(
        "> setup finished in {:.4} ms",
        (setup_elapsed.as_secs_f64() * 1000.0)
    );

    let mut sum: Vec<Vec<i32>> = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];

    if THREADS == 0 {
        simple_sum(&a, &b, &mut sum);
    } else {
        let mut handles = Vec::with_capacity(THREADS);

        for n in 0..THREADS {
            let a_ref = Arc::clone(&a);
            let b_ref = Arc::clone(&b);
            handles.push(thread::spawn(move || thread_sum(&a_ref, &b_ref, n)));
        }

        // println!("Theads: {}", handles.len());

        let joined = handles.into_iter().map(|h| h.join().unwrap());

        for s in joined {
            for (pos, e) in s.sum.iter().enumerate() {
                let index = s.indices[pos];
                sum[index] = e.clone();
            }
        }
    }

    if PRINT_MATRICES {
        print_matrix(&sum);
    }

    let total_elapsed = now.elapsed();
    println!(
        "> task finished in {:.4} ms",
        ((total_elapsed - setup_elapsed).as_secs_f64() * 1000.0)
    );
}

struct ThreadSumResult {
    sum: Vec<Vec<i32>>,
    indices: Vec<usize>,
}

fn thread_sum(a: &Matrix, b: &Matrix, thread_index: usize) -> ThreadSumResult {
    let mut sum: Vec<Vec<i32>> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();

    for x in (thread_index..MATRIX_SIZE).step_by(THREADS) {
        // println!("Thread {} processing {} column", thread_index, x);

        let mut row: Vec<i32> = Vec::with_capacity(MATRIX_SIZE);
        for j in 0..MATRIX_SIZE {
            row.push(a[x][j] + b[x][j]);
        }

        sum.push(row);
        indices.push(x);
    }
    return ThreadSumResult { sum, indices };
}

fn simple_sum(a: &Matrix, b: &Matrix, sum: &mut Matrix) {
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            sum[i][j] = a[i][j] + b[i][j];
        }
    }
}

fn generate_matrix(rng: &mut SmallRng) -> Matrix {
    let mut arr: Matrix = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];

    arr.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|elem| {
            let num = rng.gen_range(RNG_FROM..RNG_TO);
            *elem = num;
        })
    });

    return arr;
}

fn print_matrix(arr: &Matrix) {
    arr.iter().for_each(|row| {
        print!("[ ");
        row.iter().for_each(|elem| {
            print!("{:5} ", elem);
        });
        println!("]");
    });
}
