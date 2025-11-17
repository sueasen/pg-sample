use rand::Rng;

#[derive(Debug)]
pub struct InputData {
    pub input: u32,
    pub matchd: bool,
}

#[derive(Debug)]
pub struct AllData {
    pub randoms: Vec<u32>,
    pub inputs: Vec<InputData>,
    pub cnt: u32,
}

fn main() {
    println!("Hello, world!");

    let randoms = generate_random_numbers(10);
    let inputs: Vec<InputData> = generate_random_numbers(3)
        .into_iter()
        .map(|num| InputData {
            input: num,
            matchd: false,
        })
        .collect();

    let mut all_data = AllData {
        randoms,
        inputs,
        cnt: 0,
    };

    println!("randoms: {:?}", all_data.randoms);

    while !all_matchd(&all_data.inputs) {
        all_data.cnt = all_data.cnt + 1;

        println!("回数・一致した数: {}・{}", all_data.cnt, matchd_count(&all_data.inputs));
        println!("input: {:?}", input_values(&all_data.inputs));

        for indata in all_data.inputs.iter_mut() {
            if !indata.matchd && all_data.randoms.contains(&indata.input) {
                indata.matchd = true;
            } else if !indata.matchd {
                indata.input = rand::rng().random_range(0..100);
            }
        }
    }
    println!("最終回数: {}", all_data.cnt);
}

fn generate_random_numbers(n: usize) -> Vec<u32> {
    (0..n).map(|_| rand::rng().random_range(0..100)).collect()
}

fn all_matchd(inputs: &Vec<InputData>) -> bool {
    inputs.iter().all(|d| d.matchd)
}

fn matchd_count(inputs: &Vec<InputData>) -> usize {
    inputs.iter().filter(|d| d.matchd).count()
}

fn input_values(inputs: &Vec<InputData>) -> Vec<u32> {
    inputs.iter().map(|d| d.input).collect::<Vec<_>>()
}
