
#![windows_subsystem = "windows"]
use windows::Win32::System::Power::{
    ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, SetThreadExecutionState,
};

use systray::Application;


fn main() {
    // Prevent sleep/display off
    unsafe {
        let result =
            SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED);
        if result.0 == 0 {
            eprintln!("Failed to set execution state.");
            return;
        }
    }

    // Create system tray
    let mut app = Application::new().expect("Failed to create tray icon");

    // Set the tooltip
    app.set_tooltip("WAKUP - Running...")
        .expect("Failed to set tooltip");

    // Add exit menu item
    // Set icon (creates a simple square icon)

    // Your existing icon creation code...
    app.set_icon_from_file(&"C:\\Users\\masoud.panahpouri\\Desktop\\Rust Exercises\\wakup\\src\\wakup.ico".to_string()).unwrap();

    app.add_menu_separator().expect("Failed to add menu separator");

    app.add_menu_item("🚀 WAKUP - Running... ⚡", |_window| {
        Ok::<_, systray::Error>(())
    }).expect("Failed to add menu item");

    app.add_menu_separator().expect("Failed to add menu separator");

    
    

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    }).expect("Failed to add menu item");

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
        Ok::<_, systray::Error>(())
    }).expect("Failed to add menu item");

    app.add_menu_item("Version 0.0.1", |_window| {
        Ok::<_, systray::Error>(())
    }).expect("Failed to add menu item");

    app.add_menu_separator().expect("Failed to add menu separator");

    app.add_menu_item("Exit", |_| {
        std::process::exit(0);
        #[allow(unreachable_code)]
        Ok::<_, std::io::Error>(())
    })
    .expect("Failed to add menu item");

    println!("WAKUP running in background. Right-click tray icon to exit.");
    
    // Run the application
    app.wait_for_message().unwrap();
   
}   
