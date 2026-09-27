//! Tune word comparisons via Hamming distance

/// Step 0: Baseline computation --- nothing clever here
pub mod basic_str {

    /// Naive computation of Hamming distance between two strings
    pub fn dist(s1: &str, s2: &str) -> isize {
        s1.chars()
            .zip(s2.chars())
            .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
            .sum::<isize>()
            + ((s1.len() as isize) - (s2.len() as isize)).abs()
    }

    /// Compute vector of mean distances for all strings
    pub fn mean_dists(dict: &[String]) -> Vec<f64> {
        let mut dists: Vec<isize> = vec![0; dict.len()];
        for (i, w1) in dict.iter().enumerate() {
            for w2 in dict.iter() {
                dists[i] += dist(w1, w2);
            }
        }
        dists
            .iter()
            .map(|d| (*d as f64) / (dict.len() as f64))
            .collect()
    }

    /// Standardized interface
    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        mean_dists(dict)
    }
}

/// Step 1: Blocked computation
pub mod block_str {

    use super::basic_str::dist; // We can use the naive string compare
    const BSIZE: usize = 500; // Probably want a constant block size param

    /// Compute vector of mean distances for all strings (you may change
    /// the interface if you want)
    fn block_updates(d1: &[String], d2: &[String], counts: &mut [isize]) {
        let mut index = 0;
        for d1Ele in d1{
            for d2Ele in d2{
                counts[index] += dist(d1Ele,d2Ele);
            }
            index += 1;
        }
    }

    /// Blocked computation
    pub fn mean_dists(dict: &[String]) -> Vec<f64> {
        let mut counts = vec![0isize;dict.len()];
        for (d1, count_blocks) in dict.chunks(BSIZE).zip(counts.chunks_mut(BSIZE)){
            for d2 in dict.chunks(BSIZE){
                block_updates(d1, d2, count_blocks);
            }
        }
        counts.iter().map(|count| *count as f64/dict.len() as f64).collect()
    }

    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        mean_dists(dict)
    }
}

/// Step 2: Removing indirection
pub mod basic_word {

    const WSIZE: usize = 29; // You may want to fiddle with this

    /// Word storage
    pub struct Word([u8; WSIZE]);

    impl Word {
        /// Create a Word from a string
        pub fn new(s: &str) -> Self {
            let mut word = Word([0u8; WSIZE]);
            word.0[0] = s.len() as u8;
            for (index,c) in s.bytes().enumerate(){
                word.0[index+1] = c;
            }
            word
        }
    }

    /// Re-pack the dictionary in more condensed form
    fn pack_dict(dict: &[String]) -> Vec<Word> {
        dict.iter().map(|s| Word::new(s)).collect()
    }

    /// Compute the Hamming distance between two Words
    fn dist(w1: &Word, w2: &Word) -> isize {
        let mut count = 0;
        let maxlen = std::cmp::max(w1.0[0], w2.0[0]) as usize;
        for index in 1..=maxlen{
            if w1.0[index] != w2.0[index]{
                count += 1;
            }
        }
        count
    }
// let mut count = 0;
//         let len1 = w1.0[0] as usize;
//         let len2 = w2.0[0] as usize;
//         let min_len = std::cmp::min(len1, len2);
//         for index in 1..=min_len{
//             if w1.0[index] != w2.0[index]{
//                 count += 1;
//             }
//         }
//         count + (len1 as isize - len2 as isize).abs()
    /// Compute vector of mean distances for all words (pre-packed)
    pub fn mean_dists(dict: &[Word]) -> Vec<f64> {
        let mut result = Vec::new();
        for ele in dict{
            let mut sum = 0;
            for ele2 in dict{
                sum += dist(ele,ele2);
            }
            let mean = sum as f64 / dict.len() as f64;
            result.push(mean);
        }
        result
    }

    /// Compute vector of mean distances for all words    
    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        let pdict = pack_dict(dict);
        mean_dists(&pdict)
    }
}

/// Step 3: SIMD Within a Register
pub mod swar_word {

    /// Word storage
    pub struct Word([u64; 3]); // You may change the internals (eg [u64; 3])

    impl Word {
        /// Create packed word
        pub fn new(s: &str) -> Self {
            let mut word = Word([0u64; 3]);
            for (i,c) in s.bytes().enumerate(){
                let value = (c - b'a' + 1) as u64;

                let chunk = i / 12;
                let offset = i % 12;
                let shift = offset * 5;
                
                word.0[chunk] |= value << shift;
            }
            word
        }
    }

    /// Compute the Hamming distance between two Words
    fn dist(w1: &Word, w2: &Word) -> isize {
        // let diff1 = w1.0[0] ^ w2.0[0];
        // let diff2 = w1.0[1] ^ w2.0[1];
        // let diff3 = w1.0[2] ^ w2.0[2];

        // let diffs = [diff1, diff2, diff3];

        // let mut count = 0;
        // for dif in diffs{
        //     for counts in (0..60).step_by(5){
        //         let part = (dif >> counts) & 0b11111;
        //         if part != 0{
        //             count += 1
        //         }
        //     }
        // }
        // count
        let mut count = 0;

        for i in 0..3 {
            let diff = w1.0[i] ^ w2.0[i];

            let mut collapsed = diff;

            for shift in 1..5 {
                collapsed |= diff >> shift;
            }

            let mask = 0x84210842108421u64;

            count += (collapsed & mask).count_ones() as isize;
        }

        count
        
    }

    /// Compute vector of mean distances for all words (pre-packed)
    pub fn mean_dists(dict: &[Word]) -> Vec<f64> {
        let mut result = Vec::new();
        let size = dict.len();
        for ele in dict{
            let mut sum = 0;
            for ele2 in dict{

                sum += dist(ele, ele2);
            }
            let mean = sum as f64 / size as f64;
            result.push(mean);
        }
        result
        
    }

    /// Re-pack the dictionary in more condensed form
    fn pack_dict(dict: &[String]) -> Vec<Word> {
        dict.iter().map(|s| Word::new(s)).collect()
    }

    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        let pdict = pack_dict(dict);
        mean_dists(&pdict)
    }
}

/// Step 4: Optimized version!
pub mod optimized {

    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_naive_dist() {
        use super::basic_str::dist;
        assert_eq!(dist("aa", "aaaaa"), 3);
        assert_eq!(dist("aaaaa", "aa"), 3);
        assert_eq!(dist("test", "tilt"), 2);
    }

    // TODO: Add your own module tests!
}
