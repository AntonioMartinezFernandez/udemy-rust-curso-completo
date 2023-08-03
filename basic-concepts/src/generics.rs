pub fn generics() {
    /* GENERICS */
    let surface_a: Measures<f32, f32> = Measures {
        length: 32.07,
        width: 4.21,
    };

    let surface_b = Measures {
        length: 19.12,
        width: 28.21,
    };

    let diff = calculate_area_difference(surface_a, surface_b);
    println!("area difference: {}", diff)
}

struct Measures<T, V> {
    length: T,
    width: V,
}

fn calculate_area_difference(surface_a: Measures<f32, f32>, surface_b: Measures<f32, f32>) -> f32 {
    let area_a = surface_a.length * surface_a.width;
    let area_b = surface_b.length * surface_b.width;
    if area_a > area_b {
        area_a - area_b
    } else {
        area_b - area_a
    }
}
