use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut a = 0.0_f64;
    let mut b_angle = 0.0_f64;

    loop {
        let mut z = [0.0_f64; 1760];
        let mut buffer = [' '; 1760];

        for j in (0..628).step_by(7) {
            let j = j as f64 * 0.01;
            for i in (0..628).step_by(2) {
                let i = i as f64 * 0.01;
                let (sin_i, cos_i) = i.sin_cos();
                let (sin_j, cos_j) = j.sin_cos();
                let (sin_a, cos_a) = a.sin_cos();
                let (sin_b, cos_b) = b_angle.sin_cos();

                let h = cos_j + 2.0;
                let d = 1.0 / (sin_i * h * sin_a + sin_j * cos_a + 5.0);
                let t = sin_i * h * cos_a - sin_j * sin_a;

                let x = (40.0 + 30.0 * d * (cos_i * h * cos_b - t * sin_b)) as i32;
                let y = (12.0 + 15.0 * d * (cos_i * h * sin_b + t * cos_b)) as i32;
                let o = x + 80 * y;

                let n = (8.0
                    * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b
                        - sin_i * cos_j * sin_a
                        - sin_j * cos_a
                        - cos_i * cos_j * sin_b)) as usize;
                if y < 22 && y > 0 && x > 0 && x < 80 && d > z[o as usize] {
                    z[o as usize] = d;
                    buffer[o as usize] = ".,-~:;=!*#$@".chars().nth(n.clamp(0, 11)).unwrap_or(' ');
                }
            }
        }

        print!("\x1b[H");
        for (i, ch) in buffer.iter().enumerate() {
            print!("{}", ch);
            if i % 80 == 79 {
                println!();
            }
        }

        io::stdout().flush().unwrap();
        a += 0.04;
        b_angle += 0.02;
        sleep(Duration::from_millis(30));
    }
}

