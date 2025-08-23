use std::collections::HashSet;

/* This project corresponds to task number 428 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
We're running a salad restaurant! You discover some starter code
from a previous developer working at the company. The code includes:
- A Vegetable enum
- A Protein enum
- A Dressing enum

Our next goal is to build a Salad struct. Each Salad will have a
'protein', 'vegetables', and a 'dressing' field. A Salad can store
1 protein, any number of vegetables, and 1 dressing. Use a vector
to store the vegetables. Derive the Debug trait.

We need to implement the following 4 functions/methods on a Salad.
All 4 must have a complementary unit test. It's up to you whether you
want to write your tests first (TDD) or write your implementation
first. Follow the best practices for unit tests (modules, configuration,
etc). Feel free to utilize any helper crates (pretty_assertions,
rstest, etc).

First, define a 'new' constructor function that accepts a 'protein',
a 'vegetables' vector, and a 'dressing' and returns an instance of
the Salad. In the test, assert that the 3 fields of the Salad are
correctly populated.

Next, define an 'is_valid' method that returns a Boolean. Return
a true if a salad has more than 0 vegetables.

Next, define a 'calories' method that calculates the total calories
in the salad. The Vegetable, Protein, and Dressing enums all support
a 'calories' method that return the calories of the item. Remember
that 'vegetables' is a vector of multiple Vegetable values -- you'll
have to include all of them in your calculation.

Finally, define a 'has_duplicate_vegetables' method. It should
determine if the salad includes any vegetable more than once.
Return a Boolean.
*/

trait Caloric {
    fn calories(&self) -> u32;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl Salad {
    fn new(protein: Protein, vegetables: Vec<Vegetable>, dressing: Dressing) -> Salad {
        Self {
            protein: protein,
            vegetables,
            dressing,
        }
    }

    fn is_valid(&self) -> bool {
        if self.vegetables.is_empty() {
            return false;
        } else {
            return true;
        }
    }

    fn calories(&self) -> u32 {
        let mut tot_calories = self.dressing.calories() + self.protein.calories();
        for veg in &self.vegetables {
            tot_calories += veg.calories();
        }
        return tot_calories;
    }

    fn has_duplicate_vegetables(&self) -> bool {
        let mut seen = HashSet::new();
        for veg in &self.vegetables {
            if !seen.insert(veg) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper_constructor() -> Salad {
        return Salad::new(
            Protein::CrispyChicken,
            vec![Vegetable::Tomato, Vegetable::Cucumber],
            Dressing::Italian,
        ); // Tot. calories = 565
    }

    #[test]
    fn test_constructor() {
        let salad = helper_constructor();
        assert_eq!(salad.protein, Protein::CrispyChicken);
        assert_eq!(
            salad.vegetables,
            vec![Vegetable::Tomato, Vegetable::Cucumber]
        );
        assert_eq!(salad.dressing, Dressing::Italian);
    }

    #[test]
    fn test_is_valid() {
        let salad = helper_constructor();
        assert_eq!(salad.is_valid(), true);
    }

    #[test]
    fn test_calories() {
        let salad = helper_constructor();
        assert_eq!(salad.calories(), 565);
    }

    #[test]
    fn test_has_duplicate_vegetables() {
        let salad = helper_constructor();
        assert_eq!(salad.has_duplicate_vegetables(), false);
    }
}
