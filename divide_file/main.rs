// mod beta1;
// mod beta2;

// mod allmd::beta1;
// mod allmd::beta2;

// mod allmd;

// mod beta1;
// mod beta2;
mod allmd;

fn main(){
    allmd::beta1::print_beta1();
    allmd::beta2::print_beta2();
    // beta1::print_beta1();
    // beta2::print_beta2();
    allmd::print_allmd();
}