mod my;

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

macro_rules!  create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()",
                        stringify!($func_name));
        } 
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}",
                stringify!($expression),
                $expression);
    };
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right);
    };

    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right);
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    )
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // rary::public_function();
    // rary::indirect_access();
    foo();
    bar();
    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
    say_hello!();
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
