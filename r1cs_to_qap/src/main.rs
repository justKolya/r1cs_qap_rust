use std::vec;

fn mod_p(num: isize) -> isize{
    let p = 13;
    let mut res = num % p;
    if res >= 0{
        return res;
    }
    else{
        res += p;
        return res;
    }
}

struct r1cs_constraint{
    s: Vec<isize>,
    a: Vec<Vec<isize>>,
    b: Vec<Vec<isize>>,
    c: Vec<Vec<isize>>,
    n: usize,
    m: usize,
}

#[derive(Debug)]
struct QAP{
    A_x: Vec<Vec<isize>>,
    B_x: Vec<Vec<isize>>,
    C_x: Vec<Vec<isize>>,
    H_pol: Vec<isize>,
}

impl QAP {
    fn denominator(i: usize, points_x: &Vec<isize>) -> isize {
        let mut result = 1;
        let x_i = points_x[i];
        for j in (0..points_x.len()).rev() {
            if i != j{
                result *= x_i - points_x[j];
            }
        }
        mod_p(result)
    }
    
    fn mod_div_denom(denom: isize, modulo: isize) -> isize{
        let mut t = 0;
        let mut new_t = 1;
        let mut r = modulo;
        let mut new_r = denom;
        while new_r != 0{
            let q = r / new_r;
            
            let nnt = t - q*new_t;
            t = new_t;
            new_t = nnt;
    
            let nnr = r - q*new_r;
            r = new_r;
            new_r = nnr;
        }
        if r > 1 {
            panic!("no solution");
        }
        if t < 0{
            t += modulo;
        }
        t
    }
    
    fn mul_interpol(i: usize, points_x: &Vec<isize>) -> Vec<isize>{
        let mut coefficients = vec![0; points_x.len()];
        coefficients[0] = QAP::mod_div_denom(QAP::denominator(i, points_x), 13);
        let mut new_coefficients: Vec<isize>;
        for k in 0..points_x.len(){
            if k == i {continue};
            new_coefficients = vec![0; points_x.len()];
            let help = {
                if k < i{
                    k+1 
                } else {
                    k
                }
            };
            for j in (0..help).rev(){
                new_coefficients[j+1] += mod_p(coefficients[j]);
                new_coefficients[j] -= mod_p(points_x[k]*coefficients[j]);
            }
            coefficients = new_coefficients;
        }
        coefficients
    }
    
    fn lagrange(points_x: &Vec<isize>, points_y: &Vec<isize>) -> Vec<isize>{
        let mut polynomial = vec![0; points_y.len()];
        let mut coefficients: Vec<isize>;
        for i in 0..points_y.len(){
            coefficients = QAP::mul_interpol(i, points_x);
            for k in 0..points_y.len(){
                polynomial[k] += mod_p(points_y[i] * coefficients[k]);
            }
        }
        polynomial
    }
    
    #[allow(non_snake_case)]
    fn mul(A: &Vec<isize>, B: &Vec<isize>) -> Vec<isize>{
        let mut result = vec![0; A.len() + B.len() - 1];
        for i in 0..A.len(){
            for j in 0..B.len(){
                result[i+j] += mod_p(A[i]*B[j]);
            }
        }
        result
    }
    
    #[allow(non_snake_case)]
    fn subtract(A: &Vec<isize>, B: &Vec<isize>) -> Vec<isize>{
        let num = {
            if A.len() >= B.len(){
                A.len()
            } else {
                B.len()
            }
        };
        let mut result = vec![0; num];
        for i in 0..A.len(){
            result[i] += mod_p(A[i]);
        }
        for i in 0..B.len(){
            result[i] += mod_p(-B[i]);
        };
        result
    }
    
    #[allow(non_snake_case)]
    fn div(A: &Vec<isize>, B: &Vec<isize>) -> (Vec<isize>, Vec<isize>){
        let mut result = vec![0; A.len() - B.len() + 1];
        let mut remainder = A.clone();
        while remainder.len() >= B.len(){
            let leading_fac = mod_p(remainder[remainder.len()-1] / B[B.len()-1]);
            let pos = remainder.len() - B.len();
            result[pos] = leading_fac;
            let mut parameter = vec![0; pos];
            parameter.push(leading_fac);
            remainder = QAP::subtract(&remainder, &QAP::mul(B, &parameter));
            remainder.pop();
        }
        for i in 0..remainder.len(){
            remainder[i] = mod_p(remainder[i]);
        }
        (result, remainder)
    }
        
    #[allow(non_snake_case)]
    fn for_Z_x(i: usize, points_x: &Vec<isize>) -> Vec<isize>{
        let mut coefficients = vec![0; points_x.len()];
        coefficients[0] = 1;
        let mut new_coefficients: Vec<isize>;
        for k in 0..points_x.len(){
            new_coefficients = vec![0; points_x.len()];
            let help = {
                if k < i{
                    k+1 
                } else {
                    k
                }
            };
            for j in (0..help).rev(){
                new_coefficients[j+1] += mod_p(coefficients[j]);
                new_coefficients[j] -= points_x[k]*coefficients[j];
                new_coefficients[j] = mod_p(new_coefficients[j]);
            }
            coefficients = new_coefficients;
            if k == points_x.len()-1{
                coefficients[k] -= points_x.len() as isize;
                coefficients[k] = mod_p(coefficients[k]);
            }
        }
        coefficients.push(1);
        coefficients
    }

    fn create_qaps(r: r1cs_constraint) -> QAP{
        
        let n = r.n;
        let m = r.m;
        
        let mut a_pol = vec![vec![0; n]; m];
        let mut b_pol = vec![vec![0; n]; m];
        let mut c_pol = vec![vec![0; n]; m];

        let mut a_s = vec![0; n];
        let mut b_s = vec![0; n];
        let mut c_s = vec![0; n];
        
        let mut points_x: Vec<isize> = vec![];

        for i in 1..n+1{
            points_x.push(i as isize);
        };
        
        let mut points_a_y = vec![0; n];
        let mut points_b_y = vec![0; n];
        let mut points_c_y = vec![0; n];
        
        for i in 0..m{
            for j in 0..n{
                points_a_y[j] = mod_p(r.a[j][i]);
                points_b_y[j] = mod_p(r.b[j][i]);
                points_c_y[j] = mod_p(r.c[j][i]);
            }

            let a_str = QAP::lagrange(&points_x, &points_a_y);
            let b_str = QAP::lagrange(&points_x, &points_b_y);
            let c_str = QAP::lagrange(&points_x, &points_c_y);

        for j in 0..n{
            a_pol[i][j] = mod_p(a_str[j]*r.s[j]);
            b_pol[i][j] = mod_p(b_str[j]*r.s[j]);
            c_pol[i][j] = mod_p(c_str[j]*r.s[j]);
        }
    };
    let mut Z_x = vec![];
    for i in 0..n{
        Z_x = QAP::for_Z_x(i, &points_x);
    }
    println!("Z_x: {:?}", Z_x);
    
    for i in 0..n{
        for j in 0..m{
            a_s[i] += mod_p(a_pol[j][i]);
            b_s[i] += mod_p(b_pol[j][i]);
            c_s[i] += mod_p(c_pol[j][i]);
        }
    }

    let T_x = QAP::subtract(&QAP::mul(&a_s, &b_s), &c_s);
    println!("T_x: {:?}", T_x);

    let H_x = QAP::div(&T_x, &Z_x).0;
    println!("H_x: {:?}", H_x);

    let rem = QAP::div(&T_x, &Z_x).1;
    println!("rem: {:?}", rem);
    QAP{
        A_x: a_pol,
        B_x: b_pol,
        C_x: c_pol,
        H_pol: H_x,
    }
    }
}

#[allow(non_snake_case)]
fn main() {

    let r1cs = r1cs_constraint{
        s: vec![1, 3, 9, 27, 30, 35],
        a: vec![vec![0, 1, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 0],
                vec![5, 0, 0, 0, 1, 0]],        

        b: vec![vec![0, 1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0]],

        c: vec![vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1]],
        n: 4,
        m: 6,
    };
    let qap = QAP::create_qaps(r1cs);

    println!("{:?}", qap);
}