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

fn closest_index<'a>(v: &'a Vec<f32>, locations: &'a Vec<Vec<f32>>) -> usize {
    let mut closest_idx = 0usize;
    let mut closest_dist_sq = dist_sq(v, &locations[0]);
    for i in 1..locations.len() {
        let dist = dist_sq(v, &locations[i]);
        if dist < closest_dist_sq {
            closest_idx = i;
            closest_dist_sq = dist;
        }
    }

    closest_idx
}

fn bucket<'a>(data: &'a Vec<Vec<f32>>, centroids: &Vec<Vec<f32>>) -> Vec<Vec<&'a Vec<f32>>> {
    let mut buckets: Vec<Vec<&Vec<f32>>> = vec![];
    for _ in 0..centroids.len() {
        buckets.push(vec![]);
    }

    for point in data {
        let bucket_id = closest_index(point, centroids);
        buckets[bucket_id].push(point);
    }

    buckets
}

fn centroids<'a>(buckets: &Vec<Vec<&'a Vec<f32>>>, n: usize) -> Vec<Vec<f32>> {
    let mut centroids: Vec<Vec<f32>> = vec![];
    for b in buckets {
        let mut center_sum = vec![0f32; n];
        for v in b {
            center_sum = add(v, &center_sum);
        }

        let mut centroid = vec![];
        for v in center_sum {
            centroid.push(v / (b.len() as f32));
        }
        centroids.push(centroid);
    }

    centroids
}

pub fn k_means<'a>(data: &'a Vec<Vec<f32>>, k: usize, n: usize, lower: f32, upper: f32) -> Vec<Vec<f32>> {
    // make initial guesses
    let mut rng = rand::thread_rng(); // TODO: use seed
    let mut guesses: Vec<Vec<f32>> = vec![];
    for _ in 0..k {
        let mut vec = vec![];
        for _ in 0..n {
            vec.push(rng.gen_range::<f32>(lower, upper));
        }
        guesses.push(vec);
    }

    // bucket each value
    let buckets = bucket(data, &guesses);

    // find actual bucket centroids
    let c = centroids(&buckets, n);

    // re-bucket
    let final_buckets = bucket(data, &c);
    let final_centroids = centroids(&final_buckets, n);
    final_centroids
}
