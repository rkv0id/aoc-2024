pub fn run(input: &str) {
    let (mut firsts, mut seconds): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip();

    firsts.sort_unstable();
    seconds.sort_unstable();

    println!(
        "Answer 1: {}",
        firsts
            .iter()
            .zip(&seconds)
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
    );

    let mut fst = 0;
    let mut snd = 0;
    let mut score = 0;

    while fst < firsts.len() && snd < seconds.len() {
        match firsts[fst].cmp(&seconds[snd]) {
            std::cmp::Ordering::Less => {
                fst += 1;
            }
            std::cmp::Ordering::Greater => {
                snd += 1;
            }
            std::cmp::Ordering::Equal => {
                let val = firsts[fst];

                let mut freq = 0;
                while snd < seconds.len() && seconds[snd] == val {
                    freq += 1;
                    snd += 1;
                }

                score += val as usize * freq;

                while fst < firsts.len() && firsts[fst] == val {
                    fst += 1;
                }
            }
        }
    }
    println!(" Answer 2: {}", score);
}
