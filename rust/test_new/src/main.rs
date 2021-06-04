// create project in rust user cargo new _____
mod geo_map;
fn main() {
    let my_favorite_place = geo_map::get_hawaii_location();
    println!("Hello, world!");
    for x in 0..10 {
        println!("{}",x);
    }
    println!("my favorite place is at lat: {} and long: {}",
              my_favorite_place.lat, my_favorite_place.long);
    let x = 5 ;
    println!("Is 'x' 10 or 100 x ={}",x);
    println!("oasld");
}
