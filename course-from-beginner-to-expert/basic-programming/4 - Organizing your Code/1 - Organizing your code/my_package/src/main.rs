use my_package::{ product::category::Category, customer::Customer, order::Order, product::Product };
// use my_package::customer::Customer;
// use my_package::product::Product;
// use my_package::product::category::Category;
// use my_package::order::Order;

fn main() {
  let product: Product = Product::new(1, "Laptop".to_string(), 1000.0, Category::Electronics);
  let customer: Customer = Customer::new(
    1,
    String::from("Alice"),
    String::from("alic@example.com")
  );
  let order: Order = Order::new(1, product, customer, 2);
  println!("Total cost of the order: ${}", order.total_bill());
}
