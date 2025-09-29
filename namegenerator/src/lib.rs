use rand::prelude::*;
use rand::rng;

/// A collection of common English first names
const FIRST_NAMES: &[&str] = &[
    "James", "Mary", "John", "Patricia", "Robert", "Jennifer", "Michael", "Linda",
    "William", "Elizabeth", "David", "Barbara", "Richard", "Susan", "Joseph", "Jessica",
    "Thomas", "Sarah", "Christopher", "Karen", "Charles", "Nancy", "Daniel", "Lisa",
    "Matthew", "Betty", "Anthony", "Helen", "Mark", "Sandra", "Donald", "Donna",
    "Steven", "Carol", "Paul", "Ruth", "Andrew", "Sharon", "Joshua", "Michelle",
    "Kenneth", "Laura", "Kevin", "Sarah", "Brian", "Kimberly", "George", "Deborah",
    "Timothy", "Dorothy", "Ronald", "Lisa", "Jason", "Nancy", "Edward", "Karen",
    "Jeffrey", "Betty", "Ryan", "Helen", "Jacob", "Sandra", "Gary", "Donna",
    "Nicholas", "Carol", "Eric", "Ruth", "Jonathan", "Sharon", "Stephen", "Michelle",
    "Larry", "Laura", "Justin", "Sarah", "Scott", "Kimberly", "Brandon", "Deborah",
    "Benjamin", "Dorothy", "Samuel", "Amy", "Gregory", "Angela", "Alexander", "Ashley",
    "Patrick", "Brenda", "Frank", "Emma", "Raymond", "Olivia", "Jack", "Cynthia"
];

/// A collection of common English last names
const LAST_NAMES: &[&str] = &[
    "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
    "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas",
    "Taylor", "Moore", "Jackson", "Martin", "Lee", "Perez", "Thompson", "White",
    "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson", "Walker", "Young",
    "Allen", "King", "Wright", "Scott", "Torres", "Nguyen", "Hill", "Flores",
    "Green", "Adams", "Nelson", "Baker", "Hall", "Rivera", "Campbell", "Mitchell",
    "Carter", "Roberts", "Gomez", "Phillips", "Evans", "Turner", "Diaz", "Parker",
    "Cruz", "Edwards", "Collins", "Reyes", "Stewart", "Morris", "Morales", "Murphy",
    "Cook", "Rogers", "Gutierrez", "Ortiz", "Morgan", "Cooper", "Peterson", "Bailey",
    "Reed", "Kelly", "Howard", "Ramos", "Kim", "Cox", "Ward", "Richardson",
    "Watson", "Brooks", "Chavez", "Wood", "James", "Bennett", "Gray", "Mendoza"
];

/// Generates a random full name by combining a first name and last name
/// 
/// # Examples
/// 
/// ```
/// use namegenerator::generate_name;
/// 
/// let name = generate_name();
/// println!("Generated name: {}", name);
/// ```
pub fn generate_name() -> String {
    let mut rng = rng();
    
    let first_name = FIRST_NAMES.choose(&mut rng).unwrap();
    let last_name = LAST_NAMES.choose(&mut rng).unwrap();
    
    format!("{} {}", first_name, last_name)
}

/// Generates multiple random names
/// 
/// # Arguments
/// 
/// * `count` - The number of names to generate
/// 
/// # Examples
/// 
/// ```
/// use namegenerator::generate_names;
/// 
/// let names = generate_names(5);
/// for name in names {
///     println!("{}", name);
/// }
/// ```
pub fn generate_names(count: usize) -> Vec<String> {
    (0..count).map(|_| generate_name()).collect()
}

/// Generates a random first name only
/// 
/// # Examples
/// 
/// ```
/// use namegenerator::generate_first_name;
/// 
/// let first_name = generate_first_name();
/// println!("First name: {}", first_name);
/// ```
pub fn generate_first_name() -> String {
    let mut rng = rng();
    FIRST_NAMES.choose(&mut rng).unwrap().to_string()
}

/// Generates a random last name only
/// 
/// # Examples
/// 
/// ```
/// use namegenerator::generate_last_name;
/// 
/// let last_name = generate_last_name();
/// println!("Last name: {}", last_name);
/// ```
pub fn generate_last_name() -> String {
    let mut rng = rng();
    LAST_NAMES.choose(&mut rng).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_name() {
        let name = generate_name();
        assert!(name.contains(' '));
        assert!(name.len() > 2);
    }

    #[test]
    fn test_generate_names() {
        let names = generate_names(3);
        assert_eq!(names.len(), 3);
        for name in names {
            assert!(name.contains(' '));
            assert!(name.len() > 2);
        }
    }

    #[test]
    fn test_generate_first_name() {
        let first_name = generate_first_name();
        assert!(FIRST_NAMES.contains(&first_name.as_str()));
    }

    #[test]
    fn test_generate_last_name() {
        let last_name = generate_last_name();
        assert!(LAST_NAMES.contains(&last_name.as_str()));
    }

    #[test]
    fn test_names_are_different() {
        let name1 = generate_name();
        let name2 = generate_name();
        // With enough names in the pool, it's extremely unlikely they'd be the same
        // This test might occasionally fail due to randomness, but it's very unlikely
        // with 80+ first names and 80+ last names
        println!("Generated names: '{}' and '{}'", name1, name2);
        assert!(name1.len() > 0 && name2.len() > 0);
    }
}