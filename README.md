### Guessing game in RUST

#### Problem Statement

```rust
the program will generate a random integer between 1 and 100.
It will then prompt the player to enter a guess.
After a guess is entered, the program will indicate whether
the guess is too low or too high.
If the guess is correct, the game will print a congratulatory
message and exit.
```

#### Input/Output

To obtain user input and then print that input as output, we need to bring the `io`(input/output) library into the scope. The `io` library comes from the standard library (which is known as `std`):

```rust
use std::id;
```

#### What is prelude:

The prelude is the list of things that Rust automatically imports into every Rust program. It's kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.

#### Entry point:

`fn main()` is the main entry point of the program

#### Storing values in variables:

`let` is used to create a variable.

`let foo = bar;`

This creates a new variable `foo` and binds it to `bar` variable.
**Variable are immutable by default**
We use `mut` before variable name to make it mutable.

```rust
let foo = 5;  // immutable
let mut bar = 10; // mutable
```

`let mut guess = String::new();`
`String::new()` is a function that returns new instance of `String`. `String` is a string type provided by standard library that is growable, UTF-8 encoded bit of text.
The `::` syntax in the `::new` indicates that new is an associated function of `String` type. An associated function is implemented on type, rather than on the `instance` of that type. Some languages call it a static method.

This `new` function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.

The next part of the code, `.read_line(&mut guess)`, calls the `read_line` method on the standard input handle to get input from the user. We’re also passing one argument to read_line: `&mut guess`.

The `read_line` takes the user input and place it into a string, so it takes a string as an argument. The string argument needs to be mutable so the method can change the string’s content by adding the user input.

The `&` indicates that this argument is a reference. Like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

##### io::Result

```rust
io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
```

`read_line` puts what the user types into the string we’re passing it, but it also returns a value, in this case, an `io::Result`. Rust has a number of types named `Result` in its standard library: a generic `Result` as well as specific versions for submodules, such as `io::Result`. The `Result` types are [enumerations](https://doc.rust-lang.org/book/ch06-00-enums.html).

For `Result`, the variants are `Ok` or `Err`. The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

The purpose of these `Result` types is to encode error-handling information. Values of the `Result` type, like values of any type, have methods defined on them. An instance of `io::Result` has an `expect` method that you can call. If this instance of `io::Result` is an `Err` value, expect will cause the program to crash and display the message that you passed as an argument to expect.

##### Printing values with `println!` placeholders:

```rust
println!("You guessed: {}", guess);
```

```rust
fn main() {
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
}
```

#### Using a Crate to Get More Functionality

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The `rand` crate is a library crate, which contains code intended to be used in other programs.

#### Updating a Crate to Get a New Version

When you do want to update a crate, Cargo provides another command, `update`, which will ignore the Cargo.lock file.
If `rand` crate is the version `0.5.5` in your dependency, by default, Cargo will only look for versions greater than `0.5.5` and less than `0.6.0`.
If the rand crate has released two new versions, `0.5.6` and `0.6.0`, you would see the following if you ran cargo update:

```rust
$ cargo update
    Updating crates.io index
    Updating rand v0.5.5 -> v0.5.6
```

#### Crate docs:

Another neat feature of Cargo is that you can run the `cargo doc --open` command, which will build documentation provided by all of your dependencies locally and open it in your browser.

#### Match

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

```

A `match` expression is made up of arms. An `arm` consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm’s pattern. Rust takes the value given to `match` and looks through each arm’s pattern in turn.

#### Types

Rust has a strong, static type system. However, it also has type inference. When we wrote let mut guess = String::new(), Rust was able to infer that guess should be a String and didn’t make us write the type. The secret_number, on the other hand, is a number type.
**i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others.**

#### Loops

```rust
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

```

#### Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, let’s make the game ignore a non-number so the user can continue guessing.

```rust
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--

```

Switching from an `expect` call to a `match` expression is how you generally move from crashing on an error to handling the error.
Remember that parse returns a `Result` type and `Result` is an enum that has the variants `Ok` or `Err`. We’re using a match expression here, as we did with the Ordering result of the cmp method.

If `parse` is not able to turn the string into a number, it will return an `Err` value that contains more information about the error. The `Err` value does not match the `Ok(num)` pattern in the first match arm, but it does match the `Err(_)` pattern in the second arm. The `underscore`, \_, is a catchall value; in this example, we’re saying we want to match all `Err` values, no matter what information they have inside them. So the program will execute the second arm’s code, continue, which tells the program to go to the next iteration of the loop and ask for another guess. So, effectively, the program ignores all errors that parse might encounter!
