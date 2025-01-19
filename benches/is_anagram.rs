use std::sync::OnceLock;

use anagram::{
    is_anagram_ascii_array, is_anagram_ascii_array_panics, is_anagram_hashmap, is_anagram_sort,
    is_anagram_vector,
};
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

fn is_anagram_hashmap_bench0(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershi";
    c.bench_function(&format!("Hashmap test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_hashmap(word1, word2))
    });
}

fn is_anagram_hashmap_bench1(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershii";
    c.bench_function(&format!("Hashmap test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_hashmap(word1, word2))
    });
}

fn is_anagram_hashmap_bench2(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealership";
    c.bench_function(&format!("Hashmap test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_hashmap(word1, word2))
    });
}

fn is_anagram_hashmap_bench3(c: &mut Criterion) {
    let words = large_test_data();
    c.bench_function(
        &format!(
            "Hashmap test large({CAPS}) with {} and {}",
            words.0, words.1
        ),
        |b| b.iter(|| is_anagram_hashmap(&words.0, &words.0)),
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

fn is_anagram_vector_bench0(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershi";
    c.bench_function(&format!("Vector test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_vector(word1, word2))
    });
}

fn is_anagram_vector_bench1(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershii";
    c.bench_function(&format!("Vector test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_vector(word1, word2))
    });
}

fn is_anagram_vector_bench2(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealership";
    c.bench_function(&format!("Vector test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_vector(word1, word2))
    });
}

fn is_anagram_vector_bench3(c: &mut Criterion) {
    let words = large_test_data();
    c.bench_function(
        &format!("Vector test large({CAPS}) with {} and {}", words.0, words.1),
        |b| b.iter(|| is_anagram_vector(&words.0, &words.0)),
    );
}

fn is_anagram_ascii_array_bench0(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershi";
    c.bench_function(&format!("Ascii array test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_ascii_array(word1, word2))
    });
}

fn is_anagram_ascii_array_bench1(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershii";
    c.bench_function(&format!("Ascii array test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_ascii_array(word1, word2))
    });
}

fn is_anagram_ascii_array_bench2(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealership";
    c.bench_function(&format!("Ascii array test with {word1} and {word2}"), |b| {
        b.iter(|| is_anagram_ascii_array(word1, word2))
    });
}

fn is_anagram_ascii_array_bench3(c: &mut Criterion) {
    let words = large_test_data();
    c.bench_function(
        &format!(
            "Ascii array test large({CAPS}) with {} and {}",
            words.0, words.1
        ),
        |b| b.iter(|| is_anagram_ascii_array(&words.0, &words.0)),
    );
}

fn is_anagram_ascii_array_panics_bench0(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershi";
    c.bench_function(
        &format!("Ascii array panics test with {word1} and {word2}"),
        |b| b.iter(|| is_anagram_ascii_array_panics(word1, word2)),
    );
}

fn is_anagram_ascii_array_panics_bench1(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealershii";
    c.bench_function(
        &format!("Ascii array panics test with {word1} and {word2}"),
        |b| b.iter(|| is_anagram_ascii_array_panics(word1, word2)),
    );
}

fn is_anagram_ascii_array_panics_bench2(c: &mut Criterion) {
    let word1 = "leadership";
    let word2 = "dealership";
    c.bench_function(
        &format!("Ascii array panics test with {word1} and {word2}"),
        |b| b.iter(|| is_anagram_ascii_array_panics(word1, word2)),
    );
}

fn is_anagram_ascii_array_panics_bench3(c: &mut Criterion) {
    let words = large_test_data();
    c.bench_function(
        &format!(
            "Ascii array panics test large({CAPS}) with {} and {}",
            words.0, words.1
        ),
        |b| b.iter(|| is_anagram_ascii_array_panics(&words.0, &words.0)),
    );
}

criterion_group!(
    benches,
    is_anagram_hashmap_bench0,
    is_anagram_sort_bench0,
    is_anagram_vector_bench0,
    is_anagram_ascii_array_bench0,
    is_anagram_ascii_array_panics_bench0,
    is_anagram_hashmap_bench1,
    is_anagram_sort_bench1,
    is_anagram_vector_bench1,
    is_anagram_ascii_array_bench1,
    is_anagram_ascii_array_panics_bench1,
    is_anagram_hashmap_bench2,
    is_anagram_sort_bench2,
    is_anagram_vector_bench2,
    is_anagram_ascii_array_bench2,
    is_anagram_ascii_array_panics_bench2,
    is_anagram_hashmap_bench3,
    is_anagram_sort_bench3,
    is_anagram_vector_bench3,
    is_anagram_ascii_array_bench3,
    is_anagram_ascii_array_panics_bench3,
);

criterion_main!(benches);
