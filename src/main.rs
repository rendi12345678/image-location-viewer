use std::env;
use std::process::Command;

// Function to extract GPS coordinates using exiftool
fn extract_gps_from_image(image_path: &str) -> Option<(f64, f64)> {
    // Run exiftool to get the GPS data
    let output = Command::new("exiftool")
        .arg("-GPSPosition")
        .arg(image_path)
        .output()
        .expect("Failed to execute exiftool");

    // Check if the command was successful
    if !output.status.success() {
        eprintln!(
            "Error running exiftool: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Full exiftool output:\n{}", stdout); // Print the full output

    // Find the line containing GPS Position
    for line in stdout.lines() {
        if line.starts_with("GPS Position") {
            // Extract the GPS data after "GPS Position: "
            let gps_data = line.split(": ").nth(1)?; // Get the part after "GPS Position: "
            let parts: Vec<&str> = gps_data.split(", ").collect(); // Split by comma

            // Check if the GPS data format is as expected
            if parts.len() != 2 {
                eprintln!("Unexpected GPS format.");
                return None;
            }

            // Parse latitude
            let lat_part = parts[0].trim();
            let lat_tokens: Vec<&str> = lat_part.split_whitespace().collect();

            // Validate latitude tokens
            if lat_tokens.len() != 5 {
                eprintln!("Unexpected latitude format: {:?}", lat_tokens);
                return None; // Return if the format is unexpected
            }

            // Extract and parse latitude components
            let lat_deg_str = lat_tokens[0].trim(); // Degrees
            let lat_min_str = lat_tokens[2].trim_end_matches("'").trim(); // Minutes
            let lat_sec_str = lat_tokens[3].trim_end_matches("\"").trim(); // Seconds
            let lat_dir = lat_tokens[4]; // Direction

            // Debug: print the latitude token before parsing
            println!(
                "Latitude: {} degrees, {} minutes, {} seconds, direction: {}",
                lat_deg_str, lat_min_str, lat_sec_str, lat_dir
            );

            // Extract and parse latitude
            let lat_deg: f64 = lat_deg_str
                .parse()
                .expect("Failed to parse latitude degrees");
            let lat_min: f64 = lat_min_str
                .parse()
                .expect("Failed to parse latitude minutes");
            let lat_sec: f64 = lat_sec_str
                .parse()
                .expect("Failed to parse latitude seconds");

            // Parse longitude
            let lon_part = parts[1].trim();
            let lon_tokens: Vec<&str> = lon_part.split_whitespace().collect();

            // Validate longitude tokens
            if lon_tokens.len() != 5 {
                eprintln!("Unexpected longitude format: {:?}", lon_tokens);
                return None; // Return if the format is unexpected
            }

            // Extract and parse longitude components
            let lon_deg_str = lon_tokens[0].trim(); // Degrees
            let lon_min_str = lon_tokens[2].trim_end_matches("'").trim(); // Minutes
            let lon_sec_str = lon_tokens[3].trim_end_matches("\"").trim(); // Seconds
            let lon_dir = lon_tokens[4]; // Direction

            // Print the values being parsed
            println!(
                "Longitude: {} degrees, {} minutes, {} seconds, direction: {}",
                lon_deg_str, lon_min_str, lon_sec_str, lon_dir
            );

            // Extract and parse longitude
            let lon_deg: f64 = lon_deg_str
                .parse()
                .expect("Failed to parse longitude degrees");
            let lon_min: f64 = lon_min_str
                .parse()
                .expect("Failed to parse longitude minutes");
            let lon_sec: f64 = lon_sec_str
                .parse()
                .expect("Failed to parse longitude seconds");

            // Convert to decimal degrees
            let latitude = lat_deg + (lat_min / 60.0) + (lat_sec / 3600.0);
            let longitude = lon_deg + (lon_min / 60.0) + (lon_sec / 3600.0);

            // Adjust for South/West directions
            let latitude = if lat_dir == "S" { -latitude } else { latitude };
            let longitude = if lon_dir == "W" {
                -longitude
            } else {
                longitude
            };

            return Some((latitude, longitude));
        }
    }

    eprintln!("No GPS data found.");
    None
}

fn main() {
    // Get the image file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return;
    }

    let image_path = &args[1];

    // Extract GPS coordinates from the image
    if let Some((latitude, longitude)) = extract_gps_from_image(image_path) {
        println!("GPS Coordinates: {}, {}", latitude, longitude);

        // Construct the Google Maps URL
        let maps_url = format!(
            "https://www.google.com/maps/search/?api=1&query={},{}",
            latitude, longitude
        );

        // Print the URL to the console
        println!("Opening the following URL:\n{}", maps_url);

        // Open the URL using xdg-open
        if let Err(e) = Command::new("xdg-open").arg(&maps_url).spawn() {
            eprintln!("Failed to open URL in the browser: {}", e);
        }
    } else {
        eprintln!("No GPS data found in the image.");
    }
}
