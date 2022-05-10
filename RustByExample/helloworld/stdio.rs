fn main(){
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");
    println!("{subject} {verb} {object}",
            object = "food", 
            subject = "lml",
            verb = "eat");
    
    println!("{} of {:b} people know binary, the other half don't.", 16, 20);
    println!("{number:>0width$}", number=1, width = 6);
    println!("My name is {0}, {} {0}", "Bond");
    #[allow(dead_code)]
    struct Structure(i32);

   // println!("This structure '{}' won't print...", Structure(3));
    
    format!("Hello");                 // => "Hello"
    println!("{}", format!("Hello, {}!", "world"));   // => "Hello, world!"
    format!("The number is {}", 1);   // => "The number is 1"
    format!("{:?}", (3, 4));          // => "(3, 4)"
    format!("{value}", value=4);      // => "4"
    let people = "Rustaceans";
    format!("Hello {people}!", people = people);       // => "Hello Rustaceans!"
    format!("{} {}", 1, 2);           // => "1 2"
    format!("{:04}", 42);             // => 带前导零的 "0042"
    let argument = 2 + 2;
    format!("{argument}");   // => "4"
}
