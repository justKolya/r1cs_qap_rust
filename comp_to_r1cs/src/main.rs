#[derive(Debug)]
#[allow(non_camel_case_types)]
struct r1cs_constraint{
    a_comp: Vec<Vec<i128>>,
    b_comp: Vec<Vec<i128>>,
    c_comp: Vec<Vec<i128>>,
    s_vec: Vec<i128>,
    index: usize
}

#[allow(dead_code)]
impl r1cs_constraint{
    fn new(s: &Vec<i128>) -> r1cs_constraint {
        r1cs_constraint {
            a_comp: vec![vec![0i128; s.len()]; s.len()-2],
            b_comp: vec![vec![0i128; s.len()]; s.len()-2],
            c_comp: vec![vec![0i128; s.len()]; s.len()-2],
            s_vec: s.to_vec(),
            index: 0,
        }
    }
    fn create_constraint_trans_num(&mut self, oper: char, sec_comp: i128){
        match oper {
            '+' => {
                self.a_comp[self.index][0] = sec_comp;
                self.a_comp[self.index][self.index+2] = 1;
                self.b_comp[self.index][0] = 1;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index();             
            },
            '*' => {
                self.a_comp[self.index][self.index+2] = 1;
                self.b_comp[self.index][0] = sec_comp;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index();    
            }
            _ => {
                panic!("undefined symbol");
            }
        }
    }
    fn create_constraint_trans_x(&mut self, oper: char){
        match oper {
            '+' => {
                self.a_comp[self.index][1] = 1;
                self.a_comp[self.index][self.index+2] = 1;
                self.b_comp[self.index][0] = 1;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index();               
            },
            '*' => {
                self.a_comp[self.index][self.index+2] = 1;
                self.b_comp[self.index][1] = 1;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index();    
            }
            _ => {
                panic!("undefined symbol");
            }
        }
    }
    fn create_constraint_trans_trans(&mut self, oper: char){
        match oper {
            '+' => {
                self.a_comp[self.index][1] = self.s_vec[self.index+2];
                self.a_comp[self.index][self.index+2] = 1;
                self.b_comp[self.index][0] = 1;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index();
            },
            '*' => {
                self.a_comp[self.index][self.index+2] = 1;
                self.b_comp[self.index][1] = 1;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index();    
            }
            _ => {
                panic!("undefined symbol");
            }
        }
    }
    fn create_constraint_x_x(&mut self, oper: char){
        match oper{
            '+' => {
                self.a_comp[self.index][1] = 1;
                self.b_comp[self.index][0] = 2;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index();

            },
            '*' => {
                self.a_comp[self.index][1] = 1;
                self.b_comp[self.index][1] = 1;
                self.c_comp[self.index][self.index+2] = 1;
                self.check_index(); 
            }
            _ => {
                panic!("undefined symbol");
            }
        }   
    }
    
    fn print_constraints(&self){
        println!("{:?}", self);
    }

    fn check_index (&mut self) {
        if self.index != self.s_vec.len()-1 {
            self.index += 1;
        }
    }
}

fn main() {

    // R1CS for QAP
    // operator: +, *; trans = transition
    // func create_constraint_x_x(operator) => x (operator) x
    // func create_costraint_trans_x(operator) => trans (operator) x
    // func create_constraint_trans_num(operator) => trans (operatot) num

    let s: Vec<i128> = vec![1, 3, 9, 27, 30, 35];
    let mut c = r1cs_constraint::new(&s);
    c.create_constraint_x_x('*');
    c.create_constraint_trans_x('*');
    c.create_constraint_trans_x('+');
    c.create_constraint_trans_num('+', 5);
    c.print_constraints();    
}