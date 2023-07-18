use std::{io, vec,error,fmt};
use std::collections::HashMap;
use regex::Regex;
use rand::{distributions::Alphanumeric, Rng};
use num_enum::{TryFromPrimitive,IntoPrimitive};
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(IntoPrimitive,TryFromPrimitive,Debug)]
#[repr(u8)]
pub enum Group{
    Null = 0,
    Guitar = 1,
    Basketball = 2,

}
#[derive(IntoPrimitive,TryFromPrimitive,Debug)]
#[repr(u8)]
pub enum Class{
    Null = 0,
    Class1A = 1,
    Class1B = 2,
    Class1C = 3,
}
#[derive(IntoPrimitive,TryFromPrimitive,Debug)]
#[repr(u8)]
pub enum Course{
    Null = 0,
    English = 1,
    Chinese = 2,
    Math = 3,
}
#[derive(Default)]
pub struct DB{
    pub store: HashMap<i32,(String,Group,Class,Vec<Course>)>
}
#[derive(Debug, Clone)]
struct NoRecord;

impl fmt::Display for NoRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No record")
    }
}
impl error::Error for NoRecord {}

pub trait CRUD{
    fn create(&mut self, name:String,group:Group,class:Class,courses:Vec<Course>)->Result<i32>;
    fn read(&self,index:i32)->Option<&(String,Group,Class,Vec<Course>)>;
    fn update(&mut self, index:i32,name:String,group:Group,class:Class,courses:Vec<Course>)->Result<()>;
    fn delete(&mut self,index:i32) ->Result<()>;
}
impl CRUD for DB{
    fn create(&mut self,name:String,group:Group,class:Class,courses:Vec<Course>)->Result<i32>{
        let last_key = self.store.iter().last();
        if let Some((i,_))=last_key{
            let n = i+1;
            self.store.insert(n,(name,group,class,courses));
            Ok(n)
        }else{
            self.store.insert(0,(name,group,class,courses));
            Ok(0)
        }
        
    }
    fn read(&self,index:i32)->Option<&(String,Group,Class,Vec<Course>)>{
        self.store.get(&index)
    }
    fn update(&mut self,index:i32,name:String,group:Group,class:Class,courses:Vec<Course>)->Result<()>{
        if let Some(record) = self.store.get_mut(&index){
            *record = (name,group,class,courses);
            Ok(())
        }else{
            Err(NoRecord.into())
        }
    }
    fn delete(&mut self,index:i32) ->Result<()> {
        self.store.remove(&index).ok_or(NoRecord.into()).map(|_|())
    }
}

fn main() -> io::Result<()> {
    let route_pattern = r"^(create|read|update|delete)?\/(\d+)";
    let route_regex = Regex::new(route_pattern).unwrap();
    let mut db = DB::default();
    println!("type in the following format and press enter:");
    println!("create/1 to create a random student");
    println!("read/0 to read student with index 0");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if route_regex.is_match(&buffer) {
            let args:Vec<&str> = buffer.split("/").collect();
            match *args.get(0).unwrap(){
                "create"=>{
                    let name = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect();
                    let mut rng = rand::thread_rng();
                    let group = rng.gen_range(0..2) as u8;
                    let class = rng.gen_range(0..3) as u8;
                    let course = rng.gen_range(0..3) as u8;
                    let index = db.create(name, Group::try_from(group).unwrap_or(Group::Null), Class::try_from(class).unwrap_or(Class::Null), vec![Course::try_from(course).unwrap_or(Course::Null)]);
                    println!("record written in index {:?}",index);
                }
                "read"=>{
                    let d = *args.get(1).unwrap();
                    let record = db.read(d.strip_suffix("\n").unwrap_or("0").parse::<i32>().unwrap());
                    println!("record {:?}",record);
                }
                "update"=>{
                    let d = *args.get(1).unwrap();
                    let index = d.strip_suffix("\n").unwrap_or("0").parse::<i32>().unwrap();
                    let name = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect();
                    let mut rng = rand::thread_rng();
                    let group = rng.gen_range(0..2) as u8;
                    let class = rng.gen_range(0..3) as u8;
                    let course = rng.gen_range(0..3) as u8;
                    let result = db.update(index, name, Group::try_from(group).unwrap_or(Group::Null), Class::try_from(class).unwrap_or(Class::Null), vec![Course::try_from(course).unwrap_or(Course::Null)]);
                    println!("updated index {:?} {:?}",index, result);
                }
                "delete"=>{
                    let d = *args.get(1).unwrap();
                    let index = d.strip_suffix("\n").unwrap_or("0").parse::<i32>().unwrap();
                    let result = db.delete(index);
                    println!("deleted index {:?} {:?}",index, result);
                }
                _=>{
                    println!("Invalid command")
                }
            }
        }else{
            println!("Invalid command")
        }
    }
}