use falsaposicao::f2a;
use falsaposicao::f2b;
use falsaposicao::falsaposicao;

fn main() {
    let tol = 0.0001;
    let nmax:usize = 50;
    println!("F2A: ");
    falsaposicao(f2a,-2.1,-1.9,tol,nmax);
    falsaposicao(f2a,-1.1,-0.9,tol,nmax);
    falsaposicao(f2a,0.9,1.1,tol,nmax);
    falsaposicao(f2a,2.9,3.1,tol,nmax);
    println!("F2B: ");
    falsaposicao(f2b,-5.0,-4.0,tol,nmax);
    falsaposicao(f2b,-1.0,1.0,tol,nmax);
    falsaposicao(f2b,3.0,4.0,tol,nmax);
}