use num::complex::Complex;

fn main() {
    let calc = calculate_mandelbrot(
        1000, 
        -2.0,
        1.0, 
        -1.0,
        1.0,
        100, 
        24,
    ); 
    render_mandelbrot(calc);
}


fn calculate_mandelbrot(
    max_iters: usize, // maximum iterations
    x_min: f64, // box to evaluate
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize, // 
    height: usize,
    )  -> Vec<Vec<usize>> 
    {
        let mut rows = Vec::with_capacity(height);
        for moving_y in 0..height {
            let mut row = Vec::with_capacity(width);
            for moving_x in 0..width { 
                let x = x_min + (x_max - x_min) * ( moving_x as f64 / width as f64); 
                let y = y_min + (y_max - y_min) * ( moving_y as f64 / height as f64); 
                let mandelbrot_result = mandelbrot_at_point(x, y, max_iters);
                // println!("x:{} and y:{} = {}", x, y, mandelbrot_result);
                row.push(mandelbrot_result);
            }
            rows.push(row);
        }
        rows
    }

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z: Complex<f64> = Complex::new(0.0, 0.0);
    let c = Complex{ re:cx, im: cy};
    let mut iter = 0;
    while iter < max_iters {
        if z.norm() > 2.0 {
            return iter;
        } else {
            z = z * z + c;
            iter+=1;
        }
    
    }
    max_iters
}
 
fn render_mandelbrot(calculated: Vec<Vec<usize>>) {
    for row in calculated{
        for col in row {
            let chr = match col {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            print!("{}", chr);
        }
        println!("");
    }
}
 