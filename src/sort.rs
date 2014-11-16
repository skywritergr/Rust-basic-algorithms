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
		while /*i >= 0 &&*/ resultArray[i] > key {
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

pub fn selection_sort(array: Vec<uint>) -> Vec<uint> {
	let mut resultArray = array;
	let mut minVal : uint;

	for i in range(0, resultArray.len()){
		minVal = resultArray[i];
		for j in range(i+1,resultArray.len()) {
			if minVal > resultArray[j] {
				minVal = resultArray[j];
			}
		}
		*resultArray.get_mut(i) = minVal;
	}

	return resultArray;
}


//That won't work in Rust as it doesn't support recursion as of yet.
//most likely it never won't. :(
pub fn merge_sort(array: Vec<uint>) -> Vec<uint> {
	let resultArray = array;
	let mut leftArray = Vec::new();
	let mut rightArray = Vec::new();
	let middle : uint = resultArray.len()/2;

	if resultArray.len() <= 0{
		return resultArray;
	}

	for i in range(0,resultArray.len()) {
		if i < middle {
			leftArray.push(resultArray[i]);
		} else if i >= middle {
			rightArray.push(resultArray[i]);
		}
	}

	let left = merge_sort(leftArray);
	let right = merge_sort(rightArray);

	return merge(left,right);
}

pub fn merge(leftArray: Vec<uint>, rightArray: Vec<uint>) -> Vec<uint> {
	let mut result = Vec::new();
	let mut left = leftArray;
	let mut right = rightArray;

	while left.len() > 0 || right.len() > 0 {
		if left.len() > 0 && right.len() > 0 {
			if left[0] <= right[0] {
				result.push(left[0]);
				left.remove(0);
			} else {
				result.push(right[0]);
				right.remove(0);
			}
		} else if left.len() > 0 {
			result.push(left[0]);
			left.remove(0);
		} else if right.len() > 0 {
			result.push(right[0]);
			right.remove(0);
		}
	}

	return result;
}