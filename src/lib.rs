extern crate rand_core;
extern crate rand;
extern crate rand_xorshift;

fn add(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    if v1.len() != v2.len() {
        panic!("TODO: vectors are mis-sized");
    }

    v1.iter().zip(v2.iter())
        .map(|(a, b)| a + b).collect()
}

fn dist_sq(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    if v1.len() != v2.len() {
        panic!("TODO: vectors are mis-sized");
    }

    v1.iter().zip(v2.iter())
        .map(|(a, b)| a - b)
        .map(|t| t * t).sum()
}

fn closest_index<F>(v: &Vec<f32>, locations: &Vec<Vec<f32>>, dist: F) -> usize
    where F: Fn(&Vec<f32>, &Vec<f32>) -> f32 {

    let mut closest_idx = 0usize;
    let mut closest_dist_sq = dist(v, &locations[0]);
    for i in 1..locations.len() {
        let dist = dist(v, &locations[i]);
        if dist < closest_dist_sq {
            closest_idx = i;
            closest_dist_sq = dist;
        }
    }

    closest_idx
}

fn bucket_and_center<F>(data: &Vec<Vec<f32>>, centroids: &Vec<Vec<f32>>, n: usize, dist: F) -> Vec<Vec<f32>>
    where F: Fn(&Vec<f32>, &Vec<f32>) -> f32 {

    // for each data point, find the closest centroid and sum the component values
    let mut sums = vec![];
    let mut counts = vec![0; centroids.len()];
    for _ in 0..centroids.len() {
        sums.push(vec![0f32; n]);
    }
    for d in data {
        let index = closest_index(d, centroids, &dist);
        sums[index] = add(&sums[index], d);
        counts[index] = counts[index] + 1;
    }

    // divide each component value by the number of items in that bucket
    for (i, s) in sums.iter_mut().enumerate() {
        for v in s.iter_mut() {
            *v = *v / (counts[i] as f32);
        }
    }

    sums
}

pub fn k_means_with_rng_and_distance<R, F>(data: &Vec<Vec<f32>>, k: usize, n: usize, iterations: u32, lower: f32, upper: f32, rng: &mut R, dist: F) -> Vec<Vec<f32>>
    where F: Fn(&Vec<f32>, &Vec<f32>) -> f32,
          R: rand::Rng
{
    // make initial guesses
    let mut centers: Vec<Vec<f32>> = vec![];
    for _ in 0..k {
        let mut vec = vec![];
        for _ in 0..n {
            vec.push(rng.gen_range(lower, upper));
        }
        centers.push(vec);
    }

    for _ in 0..iterations {
        centers = bucket_and_center(data, &centers, n, &dist);
    }

    centers
}

pub fn k_means_with_rng<R>(data: &Vec<Vec<f32>>, k: usize, n: usize, iterations: u32, lower: f32, upper: f32, rng: &mut R) -> Vec<Vec<f32>>
    where R: rand::Rng
{
    k_means_with_rng_and_distance(data, k, n, iterations, lower, upper, rng, dist_sq)
}

pub fn k_means_with_distance<F>(data: &Vec<Vec<f32>>, k: usize, n: usize, iterations: u32, lower: f32, upper: f32, dist: F) -> Vec<Vec<f32>>
    where F: Fn(&Vec<f32>, &Vec<f32>) -> f32
{
    let mut rng = rand::thread_rng();
    k_means_with_rng_and_distance(data, k, n, iterations, lower, upper, &mut rng, dist)
}

pub fn k_means_auto(data: &Vec<Vec<f32>>, k: usize, n: usize, iterations: u32, lower: f32, upper: f32) -> Vec<Vec<f32>> {
    k_means_with_distance(data, k, n, iterations, lower, upper, dist_sq)
}

#[test]
fn k_means_with_known_seed() {
    let data: Vec<Vec<f32>> = vec![
        // converge to (0.5, 0.5)
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],

        // converge to (10.5, 10.5)
        vec![9.0, 9.0],
        vec![9.0, 10.0],
        vec![10.0, 9.0],
        vec![10.0, 10.0],
    ];
    let mut rng: rand_xorshift::XorShiftRng = rand_core::SeedableRng::seed_from_u64(42);
    let result = k_means_with_rng(&data, 2, 2, 4, 0.0, 10.0, &mut rng);
    let expected: Vec<Vec<f32>> = vec![
        vec![0.5, 0.5],
        vec![9.5, 9.5],
    ];
    assert_eq!(expected, result);
}
