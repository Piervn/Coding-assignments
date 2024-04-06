/// 
/// Radix sort
/// lexicographical sorting for strings consisted of letters from latin alphabet
///
/// There are 2 versions of algorithm:
/// - sorting strings with same length
/// - sorting strings with different length TODO
///

fn letter2num(c: char) -> usize {
    let num = c as usize - 'a' as usize;
    if num > 25 { panic!("{} is not a latin letter", c); }
    return num;
}

fn radix_sort_same_length(strings: &mut Vec<&str>) {
    if strings.is_empty() { return };

    let length = strings[0].len().clone();
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
    let mut not_empty_buckets: Vec<Vec<usize>> = vec![vec![]; 26];
    let mut letter_positions: Vec<(usize, char)> =
        strings
            .iter()
            .flat_map(|string| string.chars().enumerate())
            .collect();

    // sorting letter posotions vector
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
    let mut position_buckets: Vec<Vec<usize>> = vec![vec![]; 26];
    for letter_pos in letter_positions.iter() {
        let id = letter2num(letter_pos.1);
        position_buckets[id].push(letter_pos.0);
    }
    let mut index = 0;
    for bucket in position_buckets.iter_mut() {
        for letter_pos in bucket.iter() {
            not_empty_buckets[*letter_pos].push(index);
            index += 1;
        }
        bucket.clear();
    }

}

fn main() {
    let mut strings = vec!["xyzw", "zone", "bbba", "abcd", "bbbb", "gfre", "acdf", "qwer", "bgfd", "zsdw"];
    radix_sort_same_length(&mut strings);
    println!("{:?}", strings);

    let mut strings = vec!["xyzw", "zone", "bbba", "abcd", "bbbb", "gfre", "acdf", "qwer", "bgfd", "zsdw"];
    radix_sort(&mut strings);
}