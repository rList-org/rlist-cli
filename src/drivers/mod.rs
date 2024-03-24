pub mod onedrive;

// If you want to implement a new driver, you must implement the `CloudDriver` trait and `GetVfs` trait.


pub enum DriverConfig {
    /// `driver_type` is `onedrive`.
    Onedrive(onedrive::OnedriveConfig),
}