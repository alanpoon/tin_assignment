trait A{
    fn trailling_letter(&self)->String;
}

struct MyVec<T>(Vec<T>)where T :A;
impl <T>MyVec<T> where T:A{
    fn print(&self){
        for v in &self.0{
            println!("{}",v.trailling_letter());
        }
    }
}
struct SmallA(i32);
impl A for SmallA{
    fn trailling_letter(&self)->String{
        format!("{}{:?}",self.0,"a")
    }
}
struct SmallB(i32);
impl A for SmallB{
    fn trailling_letter(&self)->String{
        format!("{}{:?}",self.0,"b")
    }
}
struct SmallC(i32);
impl A for SmallC{
    fn trailling_letter(&self)->String{
        format!("{}{:?}",self.0,"c")
    }
}
fn main() {
    
    let my_vec = MyVec(vec![SmallA(1),SmallA(2),SmallA(3)]);
    println!("printing small a !{:?}",my_vec.print());
    
    let my_vec2 = MyVec(vec![SmallB(1),SmallB(2),SmallB(3)]);
    println!("printing small b !{:?}",my_vec2.print());
    
    let my_vec3 = MyVec(vec![SmallC(1),SmallC(2),SmallC(3)]);
    println!("printling small c !{:?}",my_vec3.print());
}