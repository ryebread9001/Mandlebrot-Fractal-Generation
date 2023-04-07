use num::complex::Complex;



fn calc_point(x: f64, y: f64, iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0}; // z starts at 0
    let c = Complex::new(x, y); // c is our given pixel location


    for i in 0..=iters {
        // if z.real > 2.0 || z.imaginary > 2.0 {
            // return i;
        // }

        if z.norm() > 2.0 {
            return i; // break if we "escaped"/diverged
        }
        z = z*z + c;
    }
    return iters;
}

fn calc_rows() -> Vec<Vec<usize>> {

    
    let height = 50;
    let width = 190;
    let mut rows = Vec::<Vec<usize>>::with_capacity(width);

    for y in 0..height {

        let mut row = Vec::<usize>::with_capacity(height);
        for x in 0..width {

            let width = width as f64;
            let height = height as f64;
            let x = x as f64;
            let y = y as f64;
            let imgx = 2.0 * (x / width) - 1.5;
            let imgy = 2.5 * (y / height) - 1.0; 
            let escaped_num = calc_point(imgx, imgy, 1000);
            
            
            row.push(escaped_num);
        }
        //print!("\n");
        rows.push(row);
    }
    return rows;
}

fn render(escaped_nums: Vec<Vec<usize>>) {
    let a = String::from(" .:-=+*#%@");
    let chars: Vec<_> = a.chars().collect();

    for row in escaped_nums {
        let mut text_row = String::with_capacity(row.len());
        for val in row {
            let character = match val{
                0 ..= 2 => chars[0],
                2 ..= 5 => chars[1],
                5 ..= 10 => chars[3],
                11 ..= 30 => chars[4],
                30 ..= 100 => chars[5],
                100 ..= 200 => chars[6],
                200 ..= 400 => chars[7],
                400 ..= 700 => chars[8],
                _ => chars[9],
            };
            text_row.push(character);
        }
        println!("{}", text_row);
    }
    


    
}

fn main() {


    render(calc_rows());
} 