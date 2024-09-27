# Roop
This library provides 2 attributes: `class` and `extends`. <br>
Combination of these attributes can 'simulate' inheritance in Rust. <br>
Note that this library and it's functions are highly experimental. <br>

As class can be marked only struct with named fields. <br>
The child class don't have to be marked as class. <br>
You can't implement `Deref` and `DerefMut` traits on child class, because `extends` actively uses them.

Extending class outside of local module is currently not working.

```rust
use roop::*;

#[class] // Marks struct as class
struct Parent {
    a: i32,
    b: i32,
}

impl Parent {
    pub fn print_all(&self) {
        println!("a: {}, b: {}", self.a, self.b);
    }

    pub fn sum_a_b(&self) -> i32 {
        self.a + self.b
    }
}

#[extends(Parent)] // Extends Parent class, inherit all fields from Parent
struct Child {
    c: i32,
}

impl Child {
    pub fn print_all(&self) { // This will override the print_all from Parent class
        println!("a: {}, b: {}, c: {}", self.a, self.b, self.c);
    }
}

fn main() {
    let parent = Parent {
        a: 1,
        b: 2,
    };

    let child = Child {
        a: 1,
        b: 2,
        c: 3,
    };

    parent.print_all(); // Output: "a: 1, b: 2" 
    child.print_all(); // Output: "a: 1, b: 2, c: 3"

    let p_sum = parent.sum_a_b();
    let c_sum = child.sum_a_b();

    assert_eq!(p_sum, c_sum); // Will pass!
}
```
