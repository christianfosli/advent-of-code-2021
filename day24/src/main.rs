use std::{collections::HashMap, error::Error};

/// Arithmetic Logic Unit
#[derive(Debug)]
pub struct Alu {
    register: HashMap<char, isize>,
    /// remanining digits of num to validate
    rem_digs: Vec<u32>,
}

impl Alu {
    fn new(val_num: usize) -> Self {
        Self {
            register: HashMap::from([('w', 0), ('x', 0), ('y', 0), ('z', 0)]),
            rem_digs: val_num
                .to_string()
                .chars()
                .rev()
                .map(|c| c.to_digit(10).unwrap())
                .collect(),
        }
    }

    fn run(&mut self, program: &str) -> Result<(), String> {
        for instr in program.lines() {
            self.do_instr(instr);
        }

        match self.register[&'z'] {
            0 => Ok(()),
            not_zero => Err(format!("Val num was not valid: exit code {}", not_zero,)),
        }
    }

    fn do_instr(&mut self, instruction: &str) {
        let op = &instruction[0..3];
        let arg1 = instruction.chars().nth(4).unwrap();

        let arg2 = instruction.split_whitespace().nth(2).map(|x| {
            match x.parse::<isize>() {
                // arg2 is a number
                Ok(n) => n,
                // arg2 is a register
                Err(_) => self.register[&x.chars().next().unwrap()],
            }
        });

        //println!("{} {} {:?}", op, arg1, arg2);

        match op {
            "inp" => {
                self.register
                    .insert(arg1, self.rem_digs.pop().unwrap() as isize);
            }
            "add" => {
                let val = self.register[&arg1] + arg2.unwrap();
                self.register.insert(arg1, val);
            }
            "mul" => {
                let val = self.register[&arg1] * arg2.unwrap();
                self.register.insert(arg1, val);
            }
            "div" => {
                let val = self.register[&arg1] / arg2.unwrap();
                self.register.insert(arg1, val);
            }
            "mod" => {
                let val = self.register[&arg1] % arg2.unwrap();
                self.register.insert(arg1, val);
            }
            "eql" => {
                if self.register[&arg1] == arg2.unwrap() {
                    self.register.insert(arg1, 1);
                } else {
                    self.register.insert(arg1, 0);
                }
            }
            _ => panic!("Unexpected operation: {}", &op),
        }

        //println!("{:?}", &self);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let program = std::fs::read_to_string("input.txt")?;

    // TODO:
    // Have to optimize this so we don't keep checking inputs that get multiplied by zero
    // Those should be changed to '9' i.e. 99_999_9XX_9XX_XXX -> only iterate over X'es

    // What is the largest model number accepted by MONAD?
    let largest = (11_111_111_111_111..99_999_999_999_999)
        .rev()
        .filter(|n| !n.to_string().chars().any(|d| d == '0'))
        .filter(|n| {
            println!("Checking {}", n);
            let mut alu = Alu::new(*n);
            let res = alu.run(&program);
            //println!("{:?}", &res);
            res.is_ok()
        })
        .next();

    println!("part 1: {:?}", largest);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_performs_simple_op() {
        let program = "inp x
mul x -1";
        let mut alu = Alu::new(1);
        alu.run(program).unwrap();
        assert_eq!(-1, alu.register[&'x']);
    }

    #[test]
    fn it_performs_sligtly_more_complicated_op() {
        // sets z to 1 if second num 3 times larger than first
        let program = "inp z
inp x
mul z 3
eql z x";
        let mut alu = Alu::new(13);
        let res = alu.run(program);
        assert!(res.is_err());
        assert_eq!(1, alu.register[&'z']);

        let mut alu = Alu::new(11);
        let res = alu.run(program);
        assert!(res.is_ok());
        assert_eq!(0, alu.register[&'z']);
    }
}
