pub fn f2a(x: f64) -> f64 {
	x.powf(4.0)-x.powf(3.0)-7.0*x.powf(2.0)+x+6.0
}
pub fn f2b(x: f64) -> f64 {
	x.powf(3.0)+1.2*x.powf(2.0)-19.0*x
}
pub fn falsaposicao(f: fn(f64)->f64,mut a:f64,mut b:f64,tol:f64,nmax:usize){
    let mut n:usize = 0;
    let mut x1:f64 = 0.0;
    println!("ITERAÇÃO\ta\t\tb\t\tRAIZ\t\tF(RAIZ)", );
    while n<nmax {
        x1 = (a*f(b)-b*f(a))/(f(b)-f(a));
        n+=1;
        if f(x1)==0.0||f64::abs(f(x1))<=tol {
            println!("{}\t{:.6}\t\t{:.6}\t{:.6}\t{:.6}", n,a,b,x1,f(x1));
            break;
        }else if f(a)*f(x1)<0.0 {
            b = x1;
        }else if f(a)*f(x1)>0.0 {
            a = x1;
        }
        println!("{}\t{:.6}\t\t{:.6}\t{:.6}\t{:.6}", n,a,b,x1,f(x1));
    }
    if f64::abs(f(x1))<=tol {
        println!("Método convergiu. Raiz aproximada encontrada.");
    }else if f64::abs(f(x1))==0.0 {
        println!("Método convergiu. Raiz exata encontrada.");
    }else if n>=nmax{
        println!("Método não convergiu. Número máximo de iterações atingido.");
    }
}