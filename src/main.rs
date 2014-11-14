extern crate algorithms;

use std::io;
use std::rand;
use algorithms::sort;

fn main() {
	let sortAlgorithms = ["Bubble Sort", "Insertion Sort", "Selection Sort", "Merge Sort"];
	let mut count = 0i;
	let mut reader = io::stdin();
	let mut randArray = Vec::new();

	println!("Choose the size of your problem");
	let mut problemSize = read_from_user();

	// if the user doesn't give a specific problem size we set it by default to 50.
	if problemSize == -1 {
		println!("There's been a problem here! Setting the size to 50");
		problemSize = 50;
	}

	//add random numbers to the vector
	for n in range(0,problemSize) {
		randArray.push((rand::random::<uint>() % 100u));
	}

	// printing the list of the available algorithms.
	for &sort in sortAlgorithms.iter() {
		println!("{}. {}", count, sort);
		count+=1;
	}

	// looping through until the user gives an input.
	loop {
		println!("Choose an algorithm to run!");
		let input = reader.read_line().ok().expect("Failed to read line");
		let input_num : Option<uint> = from_str(input.as_slice().trim());
		let num = match input_num {
		    	Some(num) => num,
		    	None => {
		    		println!("Please input a number!");
		    		continue;
		    	}
		 };
		println!("You chose algorithm {}", num);
		print_vec_str(sort::bubble_sort(randArray));
		println!("");
		return;
	}
}

fn print_vec_str(v: Vec<uint>) {
    for i in v.iter() {
        print!("{:u} ", *i)
    }
}

fn read_from_user() -> int {
	let mut reader = io::stdin();

	let input = reader.read_line().ok().expect("Failed to read line");
	let input_num : Option<int> = from_str(input.as_slice().trim());
	let num = match input_num {
	    	Some(num) => num,
	    	None => { return -1; }
	 };
	 return num;
}