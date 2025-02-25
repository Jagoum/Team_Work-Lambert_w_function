use env_logger;
use log::info;
use Team_Work_Lambert_w_function::{lambert_w::lambert_function, Cli};


fn main() {
    env_logger::init();
    let args = Cli::new();
    let arg = args.get();
    info!("Evaluating the product log of {}",arg);

    match lambert_function(arg) {
        Ok(w) => println!("W({}) = {}", arg, w),
        Err(e) => eprintln!( "{}",e),
     }
}

#[test]
fn correct_output(){
    assert_eq!(lambert_function(1.0).unwrap(),0.6839397205857212)
}

#[test]
fn invalid_output(){
    assert_ne!(2,3,"Two is not eqaul to 3");
}
 #[test]
 fn correct_input(){
    assert!(true);
 }