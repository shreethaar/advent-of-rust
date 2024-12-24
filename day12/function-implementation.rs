let locations = vec![
    Location::new(1.0, 1.0, 0.0, 0.1, SnowKg(200.0)),  
    Location::new(5.0, 5.0, 0.0, 0.05, SnowKg(100.0)), 
    Location::new(0.5, 0.5, 0.0, 1.0, SnowKg(50.0)),   
];
match find_most_dense_location(&locations) {
    Ok(location) => println!("Most dense location: {:?}", location),
    Err(e) => println!("Error: {}", e),
}

match find_nearest_location(&locations) {
    Ok(location) => println!("Nearest location with high density: {:?}", location),
    Err(e) => println!("Error: {}", e),
}
