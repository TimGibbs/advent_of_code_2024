use std::{io};
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let program = [2,4,1,3,7,5,4,7,0,3,1,5,5,5,3,0];

    let mut to_visit = VecDeque::from([(program.len(), 0)]);
    
    while let Some((pos, a)) = to_visit.pop_front() {
        for i in 0..8 {
            let n_a = a * 8 + i;
            let (_,_,_,o) = run_program(n_a, 0, 0, &program);
            if o == program[pos - 1..]
            {
                to_visit.push_back((pos - 1, n_a));
                if o.len() == program.len() {
                    println!("{}", n_a);
                    return Ok(());
                }
            }
        }
    }
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
        ..=-1_isize | 8_isize.. => panic!(),
    }
}

fn combo_operand(operand:isize,a:isize, b:isize, c:isize) -> isize {
    match operand {
        0..4 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!(),
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
