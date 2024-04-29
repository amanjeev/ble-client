use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use std::error::Error;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new()
        .await
        .expect("Cannot create manager instance.");
    let adapter_list = manager.adapters().await.expect("Cannot get adapters list.");
    if adapter_list.is_empty() {
        println!("No bluetooth adapters found.");
    }

    for adapter in adapter_list {
        println!(
            "Starting scan  on {}",
            adapter
                .adapter_info()
                .await
                .expect("Cannot get Adapter Information")
        );

        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Cannot scan BLE adapter for connected devices.");

        time::sleep(Duration::from_secs(3)).await;
        let peripherals_list = adapter
            .peripherals()
            .await
            .expect("Cannot find peripheral list.");
        if peripherals_list.is_empty() {
            println!("No peripherals found.");
        }

        for peripheral in peripherals_list {
            let peripheral_properties = peripheral
                .properties()
                .await
                .expect("Cannot get peripheral properties.");
            let peripheral_is_connected = peripheral
                .is_connected()
                .await
                .expect("Cannot check if peripheral is connected.");
            let local_name = peripheral_properties.clone()
                .unwrap()
                .local_name
                .unwrap_or("Peripheral name unknown.".to_string());
            let addr = peripheral_properties.unwrap().address;
            println!(
                "Peripheral {} (address: {}) is connected: {}.",
                &local_name, &addr, peripheral_is_connected
            );

            if !peripheral_is_connected {
                println!("Connecting to peripheral {} (address: {})", &local_name, addr);
                if let Err(err) = peripheral.connect().await {
                    println!(
                        "Error connecting to peripheral {}. Skipping this peripheral.",
                        err
                    );
                    continue;
                }
            }

            let peripheral_is_connected = peripheral
                .is_connected()
                .await
                .expect("Cannot check if peripheral is connected.");
            println!(
                "Peripheral {} is connected (again): {}.",
                &local_name, peripheral_is_connected
            );

            peripheral
                .discover_services()
                .await
                .expect("Cannot discover services for the peripheral.");
            println!("Discovering services for peripheral {}.", &local_name);
            for service in peripheral.services() {
                println!(
                    "Service UUID {}, primary {}.",
                    service.uuid, service.primary
                );
                for characteristic in service.characteristics {
                    println!("  {} characteristic found", characteristic);
                }
            }

            if peripheral_is_connected {
                println!(
                    "Was connected but now disconnecting from peripheral {}",
                    &local_name
                );
                peripheral
                    .disconnect()
                    .await
                    .expect("Error disconnecting from peripheral.");
            }
        }
    }

    Ok(())
}
