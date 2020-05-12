enum exam{
    pass(i32),
    fail(i32)
}
fn display(ee:exam){
    match ee{
        exam::pass(value)=>println!("pass value is {}",value),
        exam::fail(value)=>println!("fail value is {}",value),

    }
}

fn main(){
let exam_1=exam::pass(60);
let exam_2=exam::fail(70);
display(exam_1);
display(exam_2);

}