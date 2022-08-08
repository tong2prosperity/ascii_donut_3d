use std::f32::consts::PI;
use std::mem::size_of;
use std::ops::Rem;

pub static LUMINANCE_CHARS: [char; 12] = ['.', ',', '-','~',':', ';', '=', '!', '*', '#', '$', '@'];
static RESO:f32 = 0.05;


pub struct Donut {
    r1: f32,
    r2: f32,
    viewer_distance: u32,
    object_distance: u32,
    scr_width: usize,
    scr_height: usize,
    psi: f32,
    delta: f32,
    a_radian: f32,
    b_radian: f32,
    light_source: [f32;3]
}


impl Donut {
    pub fn new(r1: u32, r2: u32, vd: u32, od: u32, sw: usize, sl: usize) -> Self {
        Donut {
            r1: r1 as f32,
            r2: r2 as f32,
            viewer_distance: vd,
            object_distance: od,
            scr_width: sw,
            scr_height: sl,
            psi: 0.0,
            delta: 0.0,
            a_radian: 0.0,
            b_radian: 0.0,
            light_source: [1.0, 0.0, 1.0],
        }
    }

    // zx平面画圆，围绕z轴转动
    // r2*cos(psi)*cos(the) + r1*cos(psi), -r2*sin(psi)*cos(the) - r1*sin(psi), r2*sin(the)
    pub fn regulated_pixels(&self) -> Vec<Vec<char>> {
        let mut zbuf = vec![vec![-123;self.scr_height];self.scr_width];
        let mut output = vec![vec!['\0';self.scr_height];self.scr_width];
        let mut psi = 0.0f32;
        let mut theta = 0.0f32;
        let mut delta = 0.0f32;
        let two_pi = PI * 2.0;

        let wid_offset = (self.scr_width / 2) as f32;
        let hei_offset = (self.scr_height / 2) as f32;

        loop {
            psi += 0.05;
            let (sin_p, cos_p) = psi.sin_cos();
            let z = (self.r2 as f32 * sin_p) as i32;
            let z_apo = self.viewer_distance as f32 / (self.object_distance as f32 - z as f32);

            loop {
                theta += 0.05;

                let (sin_t, cos_t) = theta.sin_cos();

                let x = self.r2 as f32 * cos_t * cos_p + self.r1 as f32 * cos_t;
                let y = -1.0 * self.r2 as f32 * sin_t * cos_p - self.r1 as f32 * sin_t;
                let xp = (x * z_apo + wid_offset) as usize;
                let yp = (y * z_apo + hei_offset) as usize;


                if z > zbuf[xp][yp] {
                    zbuf[xp][yp] = z;
                    output[xp][yp] = '*';
                }

                if theta.ge(&two_pi) {
                    break;
                }
            }
            theta = 0.0;
            if psi.ge(&two_pi) {
                break;
            }
        }
        return output;
    }


    // torus rotate along with y-axis counter-clockwise
    // [-r2*sin(delta)*sin(the) + r2*cos(delta)*cos(psi)*cos(the) + r1*cos(delta)*cos(psi), -r2*sin(psi)*cos(the) - r1*sin(psi), r2*sin(delta)*cos(psi)*cos(the) + r1*sin(delta)*cos(psi) + r2*sin(the)*cos(delta)]
    pub fn next_frame(&mut self) -> Vec<Vec<char>> {
        let mut zbuf = vec![vec![-123;self.scr_height];self.scr_width];
        let mut output = vec![vec!['\0';self.scr_height];self.scr_width];
        let mut psi = 0.0f32;
        let mut theta = 0.0f32;
        let two_pi = PI * 2.0;

        let wid_offset = (self.scr_width / 2) as f32;
        let hei_offset = (self.scr_height / 2) as f32;

        self.delta += 0.17;
        let (sin_d, cos_d) = self.delta.sin_cos();

        loop {
            psi += 0.05;
            loop {
                theta += 0.05;
                let (sin_t, cos_t) = theta.sin_cos();
                let (sin_p, cos_p) = psi.sin_cos();
                let z = (self.r2 as f32 * sin_d * cos_p * cos_t + self.r1 as f32 * sin_d * cos_p + self.r2 as f32 * sin_t * cos_d) as i32;
                let z_apo = self.viewer_distance as f32 / (self.object_distance as f32 - z as f32);


                let x = -1.0 * self.r2 as f32 * sin_d * sin_t + self.r2 as f32 * cos_t * cos_p * cos_d + self.r1 as f32 * cos_d * cos_p;
                let y = -1.0 * self.r2 as f32 * sin_p * cos_t - self.r1 as f32 * sin_p;
                let xp = (x * z_apo + wid_offset) as usize;
                let yp = (y * z_apo + hei_offset) as usize;


                if z > zbuf[xp][yp] {
                    zbuf[xp][yp] = z;
                    output[xp][yp] = '*';
                }

                if theta.ge(&two_pi) {
                    break;
                }
            }
            theta = 0.0;
            if psi.ge(&two_pi) {
                break;
            }
        }
        return output;
    }


    // x = (4*cos(the) + 10)*cos(delta)*cos(psi) - 4*sin(delta)*sin(the)
    // y = ((4*cos(the) + 10)*sin(delta)*cos(psi) + 4*sin(the)*cos(delta))*sin(a) - (4*cos(the) + 10)*sin(psi)*cos(a)
    // z = ((4*cos(the) + 10)*sin(delta)*cos(psi) + 4*sin(the)*cos(delta))*cos(a) + (4*cos(the) + 10)*sin(a)*sin(psi)
    //


    // torus normal is
    // -sin(delta)*sin(the) + cos(delta)*cos(psi)*cos(the),
    // (sin(delta)*cos(psi)*cos(the) + sin(the)*cos(delta))*sin(a) - sin(psi)*cos(a)*cos(the)
    // (sin(delta)*cos(psi)*cos(the) + sin(the)*cos(delta))*cos(a) + sin(a)*sin(psi)*cos(the)
    pub fn next_frame_with_x_rotate(&mut self) -> Vec<Vec<char>> {
        let mut zbuf = vec![vec![-123;self.scr_height];self.scr_width];
        let mut output = vec![vec!['\0';self.scr_height];self.scr_width];
        let mut psi = 0.0f32;
        let mut theta = 0.0f32;
        let two_pi = PI * 2.0;

        let wid_offset = (self.scr_width / 2) as f32;
        let hei_offset = (self.scr_height / 2) as f32;

        self.delta += RESO;
        self.a_radian += RESO;
        let (sin_d, cos_d) = self.delta.sin_cos();
        let(sin_a, cos_a) = self.a_radian.sin_cos();

        loop {
            psi += 0.05;
            loop {
                theta += 0.05;
                let (sin_t, cos_t) = theta.sin_cos();
                let (sin_p, cos_p) = psi.sin_cos();
                let z = ((self.r2 * cos_t + self.r1)* sin_d * cos_p + self.r2 * sin_t * cos_d) * cos_a +
                    (self.r2 * cos_t + self.r1) * sin_a * sin_p;
                let z_apo = self.viewer_distance as f32 / (self.object_distance as f32 - z);


                let x = (self.r2 * cos_t + self.r1)* cos_d * cos_p - self.r2 * sin_d * sin_t;
                let y = ((self.r2 * cos_t + self.r1)* sin_d * cos_p + self.r2 * sin_t * cos_d)*sin_a -
                    (self.r2 * cos_t + self.r1) * sin_p * cos_a;


                let xp = (x * z_apo + wid_offset) as usize;
                let yp = (y * z_apo + hei_offset) as usize;


                if z as i32 > zbuf[xp][yp]  {
                    zbuf[xp][yp] = z as i32;
                    let normal = [-1.0 * sin_d * sin_t + cos_d * cos_p * cos_t,
                        (sin_d * cos_p * cos_t + sin_t * cos_d) * sin_a - sin_p * cos_a * cos_t,
                        (sin_d * cos_p * cos_t + sin_t * cos_d) * cos_a + sin_a * sin_p * cos_t];

                    let lumi =  dot_product_3d(normal, &self.light_source);

                    if lumi < 0.0 {
                        continue;
                    }
                    //
                    let lumi_index = (lumi * 6.5) as usize;
                    output[xp][yp] = LUMINANCE_CHARS[lumi_index];
                }

                if theta.ge(&two_pi) {
                    break;
                }
            }
            theta = 0.0;
            if psi.ge(&two_pi) {
                break;
            }
        }
        return output;
    }

    pub fn dot_product_3d(a: [f32;3], b: &[f32;3]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

}