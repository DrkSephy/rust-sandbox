/*
 * By default, the items within a module have private visibility, but this
 * can be overridden with the `pub` modifier. Only the public items of a 
 * module can be accessed from outside the module scope.
*/

fn function() {
    println!("called `function()`");
}

mod my {
    // Public function
    pub fn function() {
        println!("called `my::function()`");
    }

    // Private function
    fn private_function() {
        println!("called `my::private_function()`");
    }

    // Items can access other items in the same module
    pub fn indirect_access(){
        print!("called `my::indirect_access()`, that \n> ");

        // regardless of their visibility
        private_function();
    }

    // Public module
    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    // Private module
    mod inaccessible {
        #[allow(dead_code)]
        pub fn public_function(){
            println!("called `my::inaccessibile::public_function`");
        }
    }
}

fn main() {
    // Public items of a module can be accessed 
    my::function();

    // Modules allow disambiguation between items that have the same name
    function();

    // Private items of a module cannot be directly accessed
    // Error! `private_function` is private
    // my::private_function();

    my::indirect_access();

    // Public items inside public nested modules can be accessed from 
    // outside the parent module
    my::nested::function();

    // But private items inside public nested modules cannot be accessed
    // Error! `private_function` is private
    // my::nested::private_function();

    // Items inside private nested modules cannot be accessed, regardless
    // of their visibility
    // Error! `inaccessible` is a private module
    // my::inaccessibile::public_function();
}
