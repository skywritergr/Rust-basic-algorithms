pub fn bubble_sort(array: Vec<uint>) -> Vec<uint> {
	let mut resultArray = array;
	let mut tempVal : uint;

	for i in range(0,resultArray.len()) {
		for j in range(i+1,resultArray.len()).rev() {
			if resultArray[j] < resultArray[j-1] {
				tempVal = resultArray[j];
				// resultArray[j] = resultArray[j-1];
				*resultArray.get_mut(j) = resultArray[j-1];
				// resultArray[j-1] = tempVal;
				*resultArray.get_mut(j-1) = tempVal;
			}
		}
	}

	return resultArray;
}

pub fn insertion_sort(array: Vec<uint>) -> Vec<uint> {
	let mut resultArray = array;

	return resultArray;
}