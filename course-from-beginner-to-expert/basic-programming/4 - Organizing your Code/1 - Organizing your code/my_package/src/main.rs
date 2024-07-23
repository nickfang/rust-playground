use array_tool::vec::*;
use my_package::{ product::category::Category, customer::Customer, order::Order, product::Product };

fn main() {
  let product1: Product = Product::new(1, "Laptop".to_string(), 799.99, Category::Electronics);
  let product2: Product = Product::new(2, "T-Shirt".to_string(), 20.0, Category::Electronics);
  let product3: Product = Product::new(3, "Book".to_string(), 10.0, Category::Electronics);

  let customer: Customer = Customer::new(
    1,
    String::from("Alice"),
    String::from("alic@example.com")
  );
  let order: Order = Order::new(1, product1.clone(), customer, 2);
  println!("Total cost of the order: ${}", order.total_bill());

  let set1: Vec<&Product> = vec![&product1, &product2];
  let set2: Vec<&Product> = vec![&product2, &product3];
  let intersection = set1.intersect(set2);
  println!("The intersection is: {:?}", intersection);
}
