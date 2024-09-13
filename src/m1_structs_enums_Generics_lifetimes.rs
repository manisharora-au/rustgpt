// Enum representing different types of engines
#[derive(Debug, PartialEq)]
enum Engine {
    Petrol,
    Diesel,
    Electric,
    Hybrid,
}

// Struct representing a Vehicle, with generics and lifetimes
struct Vehicle<'a, T> {
    make: &'a str,    // A reference to the make of the vehicle (lifetime 'a)
    model: &'a str,   // A reference to the model of the vehicle (lifetime 'a)
    year: u16,        // The year the vehicle was made
    engine: Engine,   // The type of engine (from the Engine enum)
    details: T,       // Generic field to store additional details about the vehicle
}

// A generic function to return vehicle details as a string
fn vehicle_summary<'a, T: std::fmt::Debug>(vehicle: &'a Vehicle<'a, T>) -> String {
    let engine = match vehicle.engine {
        Engine::Petrol => "Petrol",
        Engine::Diesel => "Diesel",
        Engine::Electric => "Electric",
        Engine::Hybrid => "Hybrid",
    };
    format!(
        "Make: {}, Model: {}, Year: {}, Engine: {}, Details: {:?}",
        vehicle.make, vehicle.model, vehicle.year, engine, vehicle.details
    )
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_with_string_details() {
        let car_details = "This vehicle is equipped with sunroof and GPS.";
        let car = Vehicle {
            make: "Toyota",
            model: "Corolla",
            year: 2020,
            engine: Engine::Hybrid,
            details: car_details,
        };

        let summary = vehicle_summary(&car);
        assert_eq!(
            summary,
            "Make: Toyota, Model: Corolla, Year: 2020, Engine: Hybrid, Details: \"This vehicle is equipped with sunroof and GPS.\""
        );
    }

    #[test]
    fn test_vehicle_with_tuple_details() {
        let car_details = (250, "Horsepower");
        let car = Vehicle {
            make: "Ford",
            model: "Mustang",
            year: 2021,
            engine: Engine::Petrol,
            details: car_details,
        };

        let summary = vehicle_summary(&car);
        assert_eq!(
            summary,
            "Make: Ford, Model: Mustang, Year: 2021, Engine: Petrol, Details: (250, \"Horsepower\")"
        );
    }

    #[test]
    fn test_engine_equality() {
        let car1 = Vehicle {
            make: "Tesla",
            model: "Model S",
            year: 2022,
            engine: Engine::Electric,
            details: "Autopilot feature",
        };

        let car2 = Vehicle {
            make: "Nissan",
            model: "Leaf",
            year: 2022,
            engine: Engine::Electric,
            details: "Eco mode",
        };

        assert_eq!(car1.engine, car2.engine);
    }

    #[test]
    fn test_vehicle_with_diesel_engine() {
        let truck_details = (500, "Payload capacity (kg)");
        let truck = Vehicle {
            make: "Volvo",
            model: "FH16",
            year: 2018,
            engine: Engine::Diesel,
            details: truck_details,
        };

        let summary = vehicle_summary(&truck);
        assert!(summary.contains("Engine: Diesel"));
        assert_eq!(
            summary,
            "Make: Volvo, Model: FH16, Year: 2018, Engine: Diesel, Details: (500, \"Payload capacity (kg)\")"
        );
    }
}
