// src/enums.rs

pub fn enums_in_rust() {
    enum UserRole {
        BASIC,
        ADMIN,
    }

    /* You can implement methods on enums, just like structs */
    impl UserRole {
        fn describe_role(&self) {
            match self {
                UserRole::ADMIN => {
                    println!("ADMIN role: the admin role lets you access everything.")
                }
                UserRole::BASIC => {
                    println!("BASIC role: the basic role lets you only access the surface.")
                }
            }
        }
    }

    enum Website {
        LINKEDIN(String), // You can associate a value to an enum using ()
        PERSONAL(String),
        RANDOM,                   // Not all enums must have a value associated
        Other { string: String }, // You can also create a struct as another element
    }

    struct EnhancedUser {
        name: String,
        surname: String,
        age: u8,
        email: String,
        active: bool,
        role: UserRole,
        web: Website,
    }

    let user = EnhancedUser {
        name: "John".to_string(),
        surname: "Appleseed".to_string(),
        age: 88,
        email: "johnappleseed@apple.com".to_string(),
        active: true,
        role: UserRole::ADMIN,
        web: Website::LINKEDIN("johnappleseed.linkedin.com".to_string()),
    };

    fn has_access(role: UserRole) -> bool {
        match role {
            UserRole::ADMIN => return true,
            UserRole::BASIC => return false,
        } // This match structure can be compared to a switch-case structure in C
    }

    if has_access(user.role) {
        println!("The user has access!");
    } else {
        println!("The user does not have access!");
    } // Introducing if-else blocks

    let user = UserRole::BASIC;
    if let UserRole::ADMIN = user {
        // Handle this particular case. This avoids having to match all cases with match{}
    }

    let web = Website::LINKEDIN("linkedin.com".to_string());
    if let Website::LINKEDIN(website) = web {
        // Handle this particular case. This avoids having to match all cases with match{}
    }
}
