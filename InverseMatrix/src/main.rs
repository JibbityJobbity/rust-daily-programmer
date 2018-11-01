use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::thread;

//EXAMPLE

let handle: thread::JoinHandle<i32> = thread::spawn(move || {
    
});

//

type Matrix = Vec<Vec<i32>>;

fn main() -> Result<(), Box<std::error::Error>> {
    let reader = BufReader::new(File::open("matrix")?);
    let mut matrix: Matrix = Matrix::new();

    for line in reader.lines() {
        let line = line?;

        let row: Vec<i32> = line
            .split("\t")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if matrix.len() == 0 {
            matrix.resize(row.len(), Vec::new());
        }

        
        row //our vector of i32s
            .into_iter() //make an owning iterator from our vector
            .enumerate() //for every item, keep track of the index and add that index as the first value in a tuple. (the_index, item)
            .for_each(|(y_axis, val)| {
            matrix[y_axis].push(val);
        });
    }

    for row in matrix {
        //::<T>

        let mean: i32 = (row.iter().sum::<i32>() as f32 / row.len() as f32).round() as i32;

        print!("{} ", mean);
    }

    Ok(())
    // return Ok(());
}

// () == void

//-24-1 -7 3 10 17 -5 12 -8 2 6 -1 -5 2 0 12 18 11 1 0 -11 2 -11