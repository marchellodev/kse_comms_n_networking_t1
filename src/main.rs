use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::{thread, time::Instant};

const RNG_FROM: i32 = 1_000;
const RNG_TO: i32 = 9_999;
const MATRIX_SIZE: usize = 10000;
const THREADS: usize = 0;
const PRINT_MATRICES: bool = false;

// type Matrix = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];
type Matrix = Vec<Vec<i32>>;

fn main() {
    let now = Instant::now();

    println!("> program init");

    // let mut rng = rand::thread_rng();
    let mut rng = SmallRng::from_entropy();

    let a = generate_matrix(&mut rng);
    let b = generate_matrix(&mut rng);
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

    let mut sum: Matrix = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];

    if THREADS == 0 {
        simple_sum(&a, &b, &mut sum);
    } else {
        let mut handles = Vec::with_capacity(THREADS);

        for n in 0..THREADS {
            let a_ref = a.clone();
            let b_ref = b.clone();

            let builder = thread::Builder::new()
                .name("reductor".into())
                .stack_size(32 * 1024 * 1024); // 32MB of stack space
                                               // handles.push(thread::spawn(move || thread_sum(&a, &b, n)));
            handles.push(
                builder
                    .spawn(move || thread_sum(&a_ref, &b_ref, n))
                    .unwrap(),
            );
        }

        // println!("Theads: {}", handles.len());

        let joined = handles.into_iter().map(|h| h.join().unwrap());

        for s in joined {
            for i in 0..MATRIX_SIZE {
                if s[i][0] != 0 {
                    sum[i] = s[i].clone();
                }
            }
        }
    }

    if PRINT_MATRICES {
        print_matrix(&sum);
    }

    let total_elapsed = now.elapsed();
    println!(
        "> total finished in {:.4} ms",
        ((total_elapsed - setup_elapsed).as_secs_f64() * 1000.0)
    );
}

fn thread_sum(a: &Matrix, b: &Matrix, thread_index: usize) -> Matrix {
    let mut sum: Matrix = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];
    for x in (thread_index..MATRIX_SIZE).step_by(THREADS) {
        // println!("Thread {} processing {} column", thread_index, x);

        for j in 0..MATRIX_SIZE {
            sum[x][j] = a[x][j] + b[x][j];
        }
    }

    return sum;
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
            // *elem = 10;
            *elem = num;
        })
    });

    return arr;
}

fn print_matrix(arr: &Matrix) {
    arr.iter().for_each(|row| {
        print!("[ ");
        row.iter().for_each(|elem| {
            print!("{:5} ", elem); // adjust the width of the element to align the columns
        });
        println!("]");
    });
}
