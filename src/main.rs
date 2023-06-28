use proc1::*;
use lib1::HelloTrait;


macro_rules! hello_macro {
    () => {
        println!("Hello, Macro!");
    };
}

macro_rules! add_macro {
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    }
}

// Matching
macro_rules! many_rules {
    () => {
        "nothing"
    };
    ($a: expr) => {
        $a
    };
    ($a: expr, $b:expr) => {
        {
            $a+$b
        }
    };
}


// Metavariables
// block    - a block
// expr     - an expression
// ident    - an identifier
// item     - an item (function, struct, module, impl)
// lifetime - a lifetime ('foo, 'static)
// literal  - a literal (1, "Hello")
// meta     - a meta item
// pat      - a pattern
// path     - a path (::std::mem::replace)
// stmt     - a statement
// tt       - a single token tree
// ty       - a type
// vis      - a visibility qualifier

macro_rules! loop3 {
    ($block:block) => {
        let mut i = 0;
        while i < 3 {
            $block
            i = i + 1
        }
    }
}

// Repetitions
// $ ( ... ) sep rep
macro_rules! repeated_macro {
    ($($element:expr),+) => {
        {
            let mut i = 0;
            $(
                i = i + 1;
                println!("{} - {}", i, $element);
            )+
        }
    }
}

macro_rules! repeated_macro2 {
    ($($element:expr)+) => {
        {
            let mut i = 0;
            $(
                i = i + 1;
                println!("{} - {}", i, $element);
            )+
        }
    }
}

macro_rules! repeated_macro3 {
    ($($element:expr)=>+) => {
        {
            let mut i = 0;
            $(
                i = i + 1;
                println!("{} - {}", i, $element);
            )+
        }
    }
}

macro_rules! pairs {
    ($($key:expr)*, $($value:expr)*) => {
        {
            let mut i = 0;
            $(
                i = i + 1;
                println!("{} - {} => {}", i, $key, $value);
            )*
        }
    }
}

// Procedural macros
fn example1() {
    // Function-like macros
    println!(" >>> Example1");

    // Trivial cases
    empty_proc_macro!(println!("Hi 1"));
    identity_proc_macro!(println!("Hi 2"));

    // Compile time code execution
    println_proc_macro!(println!("Hi 3"));

    // Declaration of new function
    make_answer!();
    println!("Answer = {}", answer());

    // Processing the input
    string_proc_macro!(println!("Hi 4"));
}

fn example2() {
    // Attribute procedural macros
    println!(" >>> Example2");

    #[attribute_proc_macro("argument" 1)]
    fn sample_function() -> u32 {
        return 13;
    }

    #[replace_12_to_13]
    fn another_function() {
        println!("{}", 12);
    }
    println!("Another function");
    another_function();

}
fn example3() {
    // Derive macros

    #[derive(HelloTrait)]
    struct User;
    User::hello_macro();

}

fn main() {
    println!("Hello, macros!");
    // Declarative macros
    //println!(" ==== Declarative macros ====");
    //println!("Hello, world!");
    //println!["Hello, world2!"];
    //println!{"Hello, world3!"};

    //println!(" ==== Hello macro ====");
    //hello_macro!();

    //println!(" ==== Add macro ====");
    //println!("{}", add_macro!(2, 5));

    //println!(" ==== Many rules ====");
    //println!("{}", many_rules!());
    //println!("{}", many_rules!(1));
    //println!("{}", many_rules!(1, 2));

    //println!(" ==== Loop rules ====");
    //loop3!({
    //    println!(" some loop hello");
    //});

    //println!(" ==== Repeated macro ====");
    //repeated_macro!("Hello", 1, "Rust");
    //repeated_macro2!("Hello" 2 "Rust");
    //repeated_macro3!("Hello" => 3 => "Rust");

    //pairs!(1 2 3, "X" "Y" "Z");

    // Procedural macros
    //println!(" ==== Procedural macros ====");
    //example1();
    //example2();
    //example3();
}