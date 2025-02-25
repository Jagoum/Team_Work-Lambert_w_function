use env_logger;


fn main() {
    env_logger::init();

    
}

#[test]
fn correct_output(){
    assert_eq!(1,1)
}

#[test]
fn invalid_output(){
    assert_ne!(2,3,"Two is not eqaul to 3");
}
 #[test]
 fn correct_input(){
    assert!(true);
 }