extern crate libnotify;

fn main() {
    // Init libnotify
    libnotify::init("myapp");
    // Create a new notification and show it
    let n =
        libnotify::Notification::new("Summary", Some("Optional Body"), None);
    // Show the notification
    n.show().unwrap();
    // You can also use the .show() convenience method on the context
    n.update("I am another notification", None, None).unwrap();
    // Show the update notification
    n.show().unwrap();
    // We are done, deinit
    libnotify::uninit();
}
