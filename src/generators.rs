pub fn generate_random(size: usize) -> Vec<Vec<u8>> {
    use rand;
    let mut rnd_vec = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            let rnd = rand::random_range(0..=1);
            print!("{}", rnd);
            rnd_vec[i][j] = rnd;
        }
    }
    rnd_vec
}

pub fn generate_empty(size: usize) -> Vec<Vec<u8>>{
    vec![vec![0; size]; size]
}

pub fn generate_glider() -> Vec<Vec<u8>> {
    vec![
        vec![0; 10],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0; 10],
        vec![0; 10],
        vec![0; 10],
        vec![0; 10],
        vec![0; 10],
        vec![0; 10],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_random() {
        
    // }

    #[test]
    fn test_empty() {
        let size = 5;
        let empty = generate_empty(size);
        assert_eq!(empty.len(), 5);
        for i in empty {
            assert_eq!(i.len(), 5);
            let s = i.iter().fold(0u64, |sum, i|sum + (*i as u64));
            assert_eq!(s, 0);
        }
    }
}