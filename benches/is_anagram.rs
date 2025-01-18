use std::sync::OnceLock;

use anagram::{is_anagram_map, is_anagram_sort};
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, Rng};

const CAPS: usize = 100_000;

fn large_test_data() -> &'static (String, String) {
    static INSTANCE: OnceLock<(String, String)> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let mut rng = rand::thread_rng();
        let word1: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(CAPS)
            .map(char::from)
            .collect();
        let word2: String = rng
            .sample_iter(&Alphanumeric)
            .take(CAPS)
            .map(char::from)
            .collect();
        (word1, word2)
    })
}

fn is_anagram_map_bench0(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershi";
    c.bench_function(&format!("Map test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_map(word1, word2))
    });
}

fn is_anagram_map_bench1(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershii";
    c.bench_function(&format!("Map test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_map(word1, word2))
    });
}

fn is_anagram_map_bench2(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealership";
    c.bench_function(&format!("Map test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_map(word1, word2))
    });
}

fn is_anagram_map_bench3(c: &mut Criterion) {
    let words = large_test_data();
    c.bench_function(
        &format!("Map test large({CAPS}) with {} and {}", words.0, words.1),
        |b| b.iter(|| is_anagram_map(&words.0, &words.0)),
    );
}

fn is_anagram_sort_bench0(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershi";
    c.bench_function(&format!("Sort test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_sort(word1, word2))
    });
}

fn is_anagram_sort_bench1(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershii";
    c.bench_function(&format!("Sort test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_sort(word1, word2))
    });
}

fn is_anagram_sort_bench2(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealership";
    c.bench_function(&format!("Sort test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_sort(word1, word2))
    });
}

fn is_anagram_sort_bench3(c: &mut Criterion) {
    let words = large_test_data();
    c.bench_function(
        &format!("Sort test large({CAPS}) with {} and {}", words.0, words.1),
        |b| b.iter(|| is_anagram_sort(&words.0, &words.0)),
    );
}

criterion_group!(
    benches,
    is_anagram_map_bench0,
    is_anagram_sort_bench0,
    is_anagram_map_bench1,
    is_anagram_sort_bench1,
    is_anagram_map_bench2,
    is_anagram_sort_bench2,
    is_anagram_map_bench3,
    is_anagram_sort_bench3,
);

criterion_main!(benches);
