fn validate_level(table: &[( &str, Vec<&str> )], role: &str) -> Option<String> {
    for (level, roles) in table {
        for r in roles {
            if r.eq_ignore_ascii_case(role) {
                return Some(level.to_string());
            }
        }
    }
    None
}

fn main() {
    
    let table = vec![
        ("APS 1-2", vec!["Intern", "-", "Paralegal", "Placement"]),
        ("APS 3-5", vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]),
        ("APS 5-8", vec!["Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"]),
        ("EL1 8-10", vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"]),
        ("EL2 10-13", vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]),
        ("SES", vec!["CEO", "Dean", "Partner", "Principal"]),
    ];

    let query_role = "Associate";

    // Use a standard if-let or conditional check instead of match
    let result = validate_level(&table, query_role);
    if result.is_some() {
        println!("An {} holds position {}.", query_role, result.as_ref().unwrap());
    } else {
        println!("Role '{}' not found in the APS Level table.", query_role);
    }

    
    let test_roles = vec!["Intern", "Senior Lecturer", "CEO", "Classroom Teacher"];
    for role in test_roles {
        if let Some(level) = validate_level(&table, role) {
            println!("Role: {:<20} => Position: {}", role, level);
        }
    }
}