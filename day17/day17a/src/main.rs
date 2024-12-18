use std::{io};

fn main() -> io::Result<()> {
    let a = 52884621;
    let b = 0;
    let c = 0;
    

    let program = [2,4,1,3,7,5,4,7,0,3,1,5,5,5,3,0];

    let (_,_,_,output) = run_program(a,b,c,&program);
    println!("{:?}", output);

    Ok(())
}

fn run_program(a:isize, b:isize, c:isize, program:&[isize]) -> (isize, isize, isize, Vec<isize>) {
    let mut inner_a = a;
    let mut inner_b = b;
    let mut inner_c = c;
    let mut point = 0;
    let mut output : Vec<isize> = Vec::new();
    loop {
        if point >= program.len() { return (inner_a, inner_b, inner_c, output) }
        let opcode = program[point];
        let operand = program[point + 1];
        let r : Option<isize>;
        (inner_a, inner_b, inner_c, point, r) = run_fn(opcode, inner_a, inner_b, inner_c, operand, point);
        if r.is_some() {
            output.push(r.unwrap());
        }
    }
}
fn run_fn(opcode:isize, a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    match opcode {
        0 => adv(a,b,c,operand,point),
        1 => bxl(a,b,c,operand,point),
        2 => bst(a,b,c,operand,point),
        3 => jnz(a,b,c,operand,point),
        4 => bxc(a,b,c,operand,point),
        5 => out(a,b,c,operand,point),
        6 => bdv(a,b,c,operand,point),
        7 => cdv(a,b,c,operand,point),
        ..=-1_isize | 8_isize.. => (a,b,c,point, None),
    }
}

fn combo_operand(operand:isize,a:isize, b:isize, c:isize) -> isize {
    match operand {
        0..4 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => -1,
    }
}

fn adv(a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    let combo = combo_operand(operand,a,b,c);
    let r = a / 2isize.pow(combo as u32);
    (r,b,c, point+2, None)
}

fn bxl(a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    let r = b^operand;
    (a,r,c, point+2, None)
}

fn bst(a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    let combo = combo_operand(operand,a,b,c);
    let r = combo % 8;
    (a,r,c, point+2, None)
}

fn jnz(a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    if a==0 { return (a,b,c, point+2, None); }
    (a,b,c,operand as usize, None)
}

fn bxc(a:isize, b:isize, c:isize, _:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    let r = b^c;
    (a,r,c, point+2, None)
}

fn out(a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    let combo = combo_operand(operand,a,b,c);
    let r = combo % 8;
    (a,b,c, point+2, Some(r))
}

fn bdv(a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    let combo = combo_operand(operand,a,b,c);
    let r = a / 2isize.pow(combo as u32);
    (a,r,c, point+2, None)
}

fn cdv(a:isize, b:isize, c:isize, operand:isize, point: usize) -> (isize,isize,isize, usize, Option<isize>) {
    let combo = combo_operand(operand,a,b,c);
    let r = a / 2isize.pow(combo as u32);
    (a,b,r, point+2, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = run_program(0, 0, 9, &[2, 6]);
        assert_eq!(result, (0, 1, 9, Vec::new()));
    }

    #[test]
    fn case2() {
        let (_,_,_,output)  = run_program(10, 0, 0, &[5,0,5,1,5,4]);
        assert_eq!(output, [0,1,2]);
    }

    #[test]
    fn case3() {
        let (a,_,_,output)  = run_program(2024, 0, 0, &[0,1,5,4,3,0]);
        assert_eq!(a, 0);
        assert_eq!(output, [4,2,5,6,7,7,7,7,3,1,0]);
    }

    #[test]
    fn case4() {
        let (_,b,_,_)  = run_program(0, 29, 0, &[1,7]);
        assert_eq!(b, 26);
    }

    #[test]
    fn case5() {
        let (_,b,_,_)  = run_program(0, 2024, 43690, &[4,0]);
        assert_eq!(b, 44354);
    }

    #[test]
    fn case6() {
        let (_,_,_,output)  = run_program(729, 0, 0, &[0,1,5,4,3,0]);
        assert_eq!(output, [4,6,3,5,6,3,5,2,1,0]);
    }
}