fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

/// Calculate the magnitude of a vector by summing the squares of its coordinates
/// and taking the square root.
fn magnitude(vector: &[f64]) -> f64 {
    vector.into_iter().map(|e| e * e).sum::<f64>().sqrt()
}

/// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(vector: &mut [f64]) {
    let magnitude = magnitude(vector);
    for item in vector{
        *item /= magnitude;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
