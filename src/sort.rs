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
	let mut key;
	let mut i;

	for j in range(1, resultArray.len()) {
		key = resultArray[j];
		i = j - 1;
		while i >= 0 && resultArray[i] > key {
			*resultArray.get_mut(i+1) = resultArray[i];
			if i > 0 {
				i = i - 1
			} else {
				break;
			}
		}
		*resultArray.get_mut(i) = key
	}

	return resultArray;
}