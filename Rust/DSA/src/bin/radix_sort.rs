/// 
/// Radix sort
/// lexicographical sorting for strings consisted of letters from latin alphabet
/// - k - number of allowed characters (in this case 26)
/// - n - number of strings
/// 
/// There are 2 versions of the algorithm:
/// - sorting strings with same length
///     - d - length of these strings
///     - time complexity: O((n + k) * d)
/// - sorting strings with different length
///     - l_total - total length of all strings (total number of characters)
///     - time complexity: O(l_total + k)
/// 

fn letter2num(c: char) -> usize {
    let num = c as usize - 'a' as usize;
    if num > 25 { panic!("{} is not a latin letter", c); }
    return num;
}

fn radix_sort_same_length(strings: &mut Vec<&str>) {
    if strings.is_empty() { return };

    let length = strings[0].len();
    for string in strings.iter() {
        if string.len() != length { panic!("All strings must have the same length"); } 
    }

    let mut buckets: Vec<Vec<&str>> = vec![vec![]; 26];

    for i in (0..length).rev() {
        // put strings into buckets
        for string in strings.iter() {
            let id = letter2num(string.chars().nth(i).unwrap());
            buckets[id].push(string);
        }

        // collect strings from buckets 
        let mut index = 0;
        for bucket in buckets.iter_mut() {
            for string in bucket.iter() {
                strings[index] = string;
                index += 1;
            }
            bucket.clear();
        }
    }
}

fn radix_sort(strings: &mut Vec<&str>) {
    let max_length = strings
        .iter()
        .map(|string| string.len())
        .max()
        .unwrap();
    let mut letter_positions: Vec<(usize, char)> =
        strings
            .iter()
            .flat_map(|string| string.chars().enumerate())
            .collect();

    // sorting letter positions vector
    let mut letter_buckets: Vec<Vec<(usize, char)>> = vec![vec![]; 26]; 
    for letter_pos in letter_positions.iter() {
        let id = letter2num(letter_pos.1);
        letter_buckets[id].push(*letter_pos);
    }
    let mut index = 0;
    for bucket in letter_buckets.iter_mut() {
        for letter_pos in bucket.iter() {
            letter_positions[index] = *letter_pos;
            index += 1;
        }
        bucket.clear();
    }
    let mut position_buckets: Vec<Vec<(usize, char)>> = vec![vec![]; max_length];
    for letter_pos in letter_positions.iter() {
        let id = letter_pos.0;
        position_buckets[id].push(*letter_pos);
    }
    let mut index = 0;
    for bucket in position_buckets.iter_mut() {
        for letter_pos in bucket.iter() {
            letter_positions[index] = *letter_pos;
            index += 1;
        }
        bucket.clear();
    }
    
    // create not empty buckets based on letter positions
    let mut not_empty_buckets: Vec<Vec<usize>> = vec![vec![]; max_length];
    for &(letter_pos, letter) in letter_positions.iter() {
        let letter_id = letter2num(letter);
        not_empty_buckets[letter_pos].push(letter_id);
    }

    // vector containing strings of length i under index i
    let mut strings_with_len: Vec<Vec<&str>> = vec![vec![]; max_length];
    strings.iter().for_each(|&s| strings_with_len[s.len() - 1].push(s));

    let mut buckets: Vec<Vec<&str>> = vec![vec![]; 26];

    // sorting with buckets
    let mut result: Vec<&str> = vec![];
    for i in (0..max_length).rev() {
        result.splice(0..0, strings_with_len[i].iter().copied());
        for string in result.iter() {
            let id = letter2num(string.chars().nth(i).unwrap());
            buckets[id].push(string);
        }
        
        let mut index = 0;
        for bucket in buckets.iter_mut() {
            for string in bucket.iter() {
                result[index] = string;
                index += 1;
            }
            bucket.clear();
        }
    }

    *strings = result;
}

fn main() {
    let mut strings = vec!["xyzw", "zone", "bbba", "abcd", "bbbb", "gfre", "acdf", "qwer", "bgfd", "zsdw"];
    radix_sort_same_length(&mut strings);
    println!("radix sort same length: {:?}", strings);

    let mut strings = vec!["abcdefg", "vx", "ukj", "a", "qwertyuiop", "uka"];
    radix_sort(&mut strings);
    println!("radix sort: {:?}", strings);
}