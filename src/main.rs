use leetcode::problems::kth_largest_stream;
use leetcode::problems::lfu_cache;
use leetcode::problems::lru_cache;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args.get(1).expect("Usage: leetcode <problem_name>");

    match problem.as_str() {
        "lru_cache" => lru_cache::run(),
        "lfu_cache" => lfu_cache::run(),
        "kth_largest_stream" => kth_largest_stream::run(),
        _ => eprintln!("Unknown or unimplemented problem: {}", problem),
    }
}
