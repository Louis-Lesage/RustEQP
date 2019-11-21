use std::thread;
use std::time::SystemTime;

fn main() {
    println!("All the solutions are : ");
    let now = SystemTime::now();

    queen(Vec::with_capacity(8));

    let elapsed_nanos = now.elapsed().unwrap().as_nanos();
    println!("time elapsed in nanos : {}", elapsed_nanos);
}

//Not faster. The time required to start all the threads ends
// up being bigger then just do the function recursively
#[allow(dead_code)]
fn queen_multi(curr_sol: Vec<i32>) {
    if curr_sol.len() == 8 {
        println!("{}", vec_to_string(&curr_sol));
        return;
    }

    let mut vec_pool = Vec::new();

    for i in 0..8 {
        let mut poss_sol = curr_sol.clone();
        poss_sol.push(i);
        if valid(&poss_sol) {
            vec_pool.push(thread::spawn(move || {
                queen(poss_sol);
            }));
        }
    }

    for handle in vec_pool {
        handle.join().unwrap();
    }
}

fn queen(curr_sol: Vec<i32>) {
    if curr_sol.len() == 8 {
        println!("{}", vec_to_string(&curr_sol));
        return;
    }

    for i in 0..8 {
        let mut poss_sol = curr_sol.clone();
        poss_sol.push(i);
        if valid(&poss_sol) {
            queen(poss_sol);
        }
    }
}

fn valid(sol: &Vec<i32>) -> bool {
    let length = sol.len();

    for i in 0..length {
        let elem_i = sol.get(i).unwrap();

        for j in (i + 1)..length {
            // Start the validation at the next queen.
            let elem_j = sol.get(j).unwrap();

            if elem_i == elem_j {
                return false; //if they are on the same row
            } else if (elem_i - elem_j).abs() == ((j - i) as i32).abs() {
                return false; //if they are on the same diagonal
            } //They can't be on the same column by representation design
        }
    }

    return true;
}

fn vec_to_string(sol: &Vec<i32>) -> String {
    let sol_string: Vec<String> = sol.into_iter().map(|x| x.to_string()).collect();
    return sol_string.join(", ");
}

#[test]
fn test_validity() {
    assert!(valid(&vec![1, 3, 5, 7, 2, 0, 6, 4]));
    assert!(valid(&vec![0, 6, 3, 5, 7, 1, 4, 2]));
    assert!(!valid(&vec![0, 2, 3, 4, 5, 6, 7, 1]));
    assert!(!valid(&vec![1, 4, 2, 7, 5, 4, 4, 6]));
    assert!(!valid(&vec![1, 3, 5, 7, 2, 0, 6, 1]));
}

#[test]
fn test_vec_to_string() {
    assert_eq!(
        "1, 3, 5, 7, 2, 0, 6, 4",
        vec_to_string(&vec![1, 3, 5, 7, 2, 0, 6, 4])
    );
    assert_eq!(
        "0, 6, 3, 5, 7, 1, 4, 2",
        vec_to_string(&vec![0, 6, 3, 5, 7, 1, 4, 2])
    );
}
