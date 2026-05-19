use rayon::prelude::*;

pub fn parallel_sum(data: &[f64]) -> f64 {
    data.par_iter().sum()
}

pub fn parallel_square(data: &mut [f64]) {
    data.par_iter_mut().for_each(|x| *x = (*x) * (*x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel() {
        let mut data = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(parallel_sum(&data), 10.0);
        
        parallel_square(&mut data);
        assert_eq!(data, vec![1.0, 4.0, 9.0, 16.0]);
    }
}
