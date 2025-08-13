use std::thread;
use std::time::Duration;

// IoT Device Struct to simulate device readings
struct IoTDevice {
    device_id: String,
    temperature: f64,
    humidity: f64,
}

// Implementing IoTDevice with a method to simulate device readings
impl IoTDevice {
    fn new(device_id: &str) -> Self {
        IoTDevice {
            device_id: device_id.to_string(),
            temperature: 0.0,
            humidity: 0.0,
        }
    }

    fn simulate_readings(&mut self) {
        // Simulate temperature readings
        self.temperature = 20.0 + (rand::random::<f64>() * 2.0);

        // Simulate humidity readings
        self.humidity = 50.0 + (rand::random::<f64>() * 10.0);
    }

    fn print_readings(&self) {
        println!("Device ID: {}", self.device_id);
        println!("Temperature: {:.2}°C", self.temperature);
        println!("Humidity: {:.2}%", self.humidity);
        println!("------------------------");
    }
}

fn main() {
    let mut device1 = IoTDevice::new("Device-1");
    let mut device2 = IoTDevice::new("Device-2");

    loop {
        device1.simulate_readings();
        device2.simulate_readings();

        device1.print_readings();
        device2.print_readings();

        thread::sleep(Duration::from_millis(1000));
    }
}