extern crate rand;

use rand::Rng;

fn add(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    if v1.len() != v2.len() {
        panic!("TODO: vectors are mis-sized");
    }

    let mut result = vec![];
    for i in 0..v1.len() {
        result.push(v1[i] + v2[i]);
    }

    result
}

fn dist_sq(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    if v1.len() != v2.len() {
        panic!("TODO: vectors are mis-sized");
    }

    let mut sum = 0f32;
    for i in 0..v1.len() {
        let t = v1[i] - v2[i];
        sum = sum + (t * t);
    }

    sum
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

pub fn k_means_with_distance<F>(data: &Vec<Vec<f32>>, k: usize, n: usize, iterations: u32, lower: f32, upper: f32, dist: F) -> Vec<Vec<f32>>
    where F: Fn(&Vec<f32>, &Vec<f32>) -> f32 {

    // make initial guesses
    let mut rng = rand::XorShiftRng::new_unseeded(); // TODO: use seed
    let mut centers: Vec<Vec<f32>> = vec![];
    for _ in 0..k {
        let mut vec = vec![];
        for _ in 0..n {
            vec.push(rng.gen_range::<f32>(lower, upper));
        }
        centers.push(vec);
    }

    for _ in 0..iterations {
        centers = bucket_and_center(data, &centers, n, &dist);
    }

    centers
}

pub fn k_means(data: &Vec<Vec<f32>>, k: usize, n: usize, iterations: u32, lower: f32, upper: f32) -> Vec<Vec<f32>> {
    k_means_with_distance(data, k, n, iterations, lower, upper, dist_sq)
}
