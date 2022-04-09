mod linked_list;
use linked_list::list;

fn main() {
    let mut linked_list: list::LinkedList<u16> = list::LinkedList::new();
    linked_list.push(1);
    linked_list.push(2);
    linked_list.push(3);
    println!("{:#?}", linked_list);
    linked_list.pop();
    println!("{:#?}", linked_list);
    println!("Linked List Length: {}", linked_list.len());
}