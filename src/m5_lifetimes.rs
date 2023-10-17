#[allow(dead_code, unused_variables)]
fn example0() {
    let r;

    let x = 5;
    r = &x;

    println!("r: {}", r);
}

#[allow(dead_code, unused_variables)]
fn example1() {
    // Allocate space in memory
    let highest_age: i32;

    // Initialize vars
    let alice_age: i32 = 20; // 'a
    let bob_age: i32 = 21; // 'b : 'a 有効期限がaと一緒

    // Call function
    highest_age = lagest(&alice_age, &bob_age);

    println!("Highest age is {}", highest_age);

    fn lagest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1 // 逆参照
        } else {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example2() {
    // Allocate space in memory
    let highest_age: &i32;
    let new_value: i32;

    // Initialize vars
    let alice_age: i32 = 20; // 'a

    {
        let bob_age: i32 = 21; // 'b : 'a 有効期限がaと一緒
                               // Call function
        highest_age = lagest(&alice_age, &bob_age);
        new_value = *highest_age;
    } // 'b out of scope

    println!("New value is {}", new_value);

    fn lagest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_3_generics() {
    // Allocate space in memory
    let highest_age: &i32;
    let new_value: i32;

    // Initialize vars
    let alice_age: i32 = 20; // 'a

    {
        let bob_age: i32 = 21; // 'b : 'a 有効期限がaと一緒
                               // Call function
        highest_age = lagest::<i32>(&alice_age, &bob_age);
        new_value = *highest_age;
    } // 'b out of scope

    println!("New value is {}", new_value);

    fn lagest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
struct Person<'p, 'q> {
    name: &'p str,
    points: &'q f32,
}

#[allow(dead_code, unused_variables)]
fn example_4_struct() {
    // Allocate space in memory
    let highest_points: &f32;
    let new_value: f32;

    // Initialize vars
    let alice: Person = Person {
        name: "alice",
        points: &50.2,
    };

    {
        let bob: Person = Person {
            name: "bob",
            points: &40.5,
        };
        // Call function
        highest_points = lagest::<f32>(&alice.points, &bob.points);
        new_value = *highest_points;
    } // 'b out of scope

    println!("New value is {}", new_value);

    fn lagest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_lifetimes() {}
}
