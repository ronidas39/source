enum exam{
    pass(i32),
    fail(i32)
}
fn get_value(ee:exam)->i32{
    match ee{
        exam::pass(value)=>value,
        exam::fail(value)=>value,
    }
}
fn main(){
let exam_1=exam::pass(70);
let exam_2=exam::fail(50);
let x=get_value(exam_1);
let y=get_value(exam_2);
if(x>0){
println!("pass value of the exam is {}",x);
}
//if(y>0){
    //println!("fail value of the exam is {}",y);
    //}

}