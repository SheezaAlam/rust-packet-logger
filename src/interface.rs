use pnet::datalink;


pub fn list_interfaces() {
for iface in datalink::interfaces() {
println!("{} - {}", iface.name, iface.description);
}
}