use std::collections::HashMap;

#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    fn new(name: String, description: String, price: f64, quantity: i32) -> Self {
        Product {
            name,
            description,
            price,
            quantity,
        }
    }
}

enum InventoryAction {
    Add,
    Edit,
    Delete,
    Report,
}

fn main() {
    let mut inventory: HashMap<String, Product> = HashMap::new();
    let mut current_action = InventoryAction::Report;

    loop {
        match current_action {
            InventoryAction::Add => add_product(&mut inventory),
            InventoryAction::Edit => edit_product(&mut inventory),
            InventoryAction::Delete => delete_product(&mut inventory),
            InventoryAction::Report => println!("{}", generate_report(&inventory)),
        }

        // Prompt user for next action
        println!("\nAvailable actions:");
        println!("1. Add product");
        println!("2. Edit product");
        println!("3. Delete product");
        println!("4. Generate report");
        println!("5. Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");

        match input.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => current_action = InventoryAction::Add,
                2 => current_action = InventoryAction::Edit,
                3 => current_action = InventoryAction::Delete,
                4 => current_action = InventoryAction::Report,
                5 => break,
                _ => println!("Invalid choice."),
            },
            Err(_) => println!("Invalid input."),
        }
    }

    println!("Exiting inventory management system.");
}

fn add_product(inventory: &mut HashMap<String, Product>) {
    let mut name = String::new();
    let mut description = String::new();
    let mut price: f64 = 0.0;
    let mut quantity: i32 = 0;

    println!("Enter product name:");
    std::io::stdin().read_line(&mut name).expect("Error reading input");
    name = name.trim().to_string();

    println!("Enter product description:");
    std::io::stdin().read_line(&mut description).expect("Error reading input");
    description = description.trim().to_string();

    loop {
        println!("Enter product price:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");

        match input.trim().parse::<f64>() {
            Ok(value) => {
                price = value;
                break;
            },
            Err(_) => println!("Invalid price. Please enter a number."),
        }
    }

    loop {
        println!("Enter product quantity:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");

        match input.trim().parse::<i32>() {
            Ok(value) => {
                quantity = value;
                break;
            },
            Err(_) => println!("Invalid quantity. Please enter a number."),
        }
    }

    inventory.insert(name.clone(), Product::new(name, description, price, quantity));
    println!("Product added successfully.");
}

fn edit_product(inventory: &mut HashMap<String, Product>) {
    let mut product_name = String::new();

    println!("Enter the name of the product to edit:");
    std::io::stdin().read_line(&mut product_name).expect("Error reading input");
    product_name = product_name.trim().to_string();

    if !inventory.contains_key(&product_name) {
        println!("Product not found.");
        return;
    }

    let mut new_name = String::new();
    let mut new_description = String::new();
    let mut new_price: f64 = 0.0;
    let mut new_quantity: i32 = 0;

    println!("Enter new product name (leave blank to keep
