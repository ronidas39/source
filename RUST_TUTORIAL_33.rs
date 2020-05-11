#[derive(Debug)]
enum exam{
    pass(i32),
    fail(i32)
}

impl exam{
    fn check_marks(&self,cs:i32)->bool{
        if let &exam::pass(value)=self{
            value>cs
        }
        else{
            panic!("call to exam::pass() failed on {:?}",self);
        }

    }
}

fn main(){
 let check_score:i32=50;
 let exam_1=exam::pass(40);
 let x=exam_1.check_marks(check_score);
 if (x){
     println!("score is eligible for enrollment");
 }
 else{
     println!("not eligibile")
 }