fn main() {
    println!("Hello, world!");
    another_function();
    testing_params_for_fun(5);
    print_labeled_measurement(5, 'm');
}

fn another_function() {
    println!("I am having so. much. fun.");
}

fn testing_params_for_fun(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}