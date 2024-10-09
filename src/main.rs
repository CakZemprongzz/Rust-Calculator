use regex::Regex;
use gtk::{prelude::*, Application, ApplicationWindow, Button, ListBox, Box as GtkBox, Orientation, Entry};
use gtk::glib;

const APP_ID: &str = "org.rust.calc";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {

    let entry = Entry::builder()
        .placeholder_text("Enter calculation (e.g., 2 + 2)")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();


    let button_display = Button::builder()
        .label("Calculate")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_clear = Button::builder()
        .label("Clear")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let list_box = ListBox::new();

    button_display.connect_clicked({
        let list_box = list_box.clone();
        let entry = entry.clone();
        move |_| {
            let text = entry.text().to_string();
            let result = cal(text.clone());
            let label = gtk::Label::new(Some(&result));
            list_box.append(&label);
        }
    });

    button_clear.connect_clicked({
        let list_box = list_box.clone();
        let entry = entry.clone();
        move |_| {
            while let Some(row) = list_box.last_child() {
                list_box.remove(&row);
            }
            entry.set_text("");
        }
    });

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.append(&entry);          
    vbox.append(&button_display); 
    vbox.append(&button_clear);   
    vbox.append(&list_box);       

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calculator ")
        .default_width(400)
        .child(&vbox) 
        .build();

    window.present();
}

fn cal(number: String) -> String {

    let re = Regex::new(r"(\d+)\s*([\+\-\*/])\s*(\d+)").unwrap();

    if let Some(caps) = re.captures(&number) {
        let number_1: i32 = caps[1].parse().unwrap_or(0);
        let operation = &caps[2];
        let number_2: i32 = caps[3].parse().unwrap_or(0);

        let result = match operation {
            "+" => number_1 + number_2,
            "-" => number_1 - number_2,
            "*" => number_1 * number_2,
            "/" => {
                if number_2 == 0 {
                    return "Cannot divide by zero!".to_string();
                }
                number_1 / number_2
            }
            _ => return format!("Unknown operator: {}", operation),
        };

        return result.to_string();
    } else {
        return "Invalid input format. Expected format: <number><operator><number>".to_string();
    }
}

