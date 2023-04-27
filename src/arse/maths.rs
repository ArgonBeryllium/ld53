pub fn lerp<T:
	Copy +
	std::ops::Add<T, Output = T> +
	std::ops::Sub<T, Output = T> +
	std::ops::Mul<f32, Output = T>>
	(a : T, b : T, t : f32) -> T {
	a + (b - a)*t
}
