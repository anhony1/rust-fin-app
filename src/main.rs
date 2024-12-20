// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bill;

use bill::print_hello;

use std::error::Error;

use slint::SharedString;

slint::include_modules!();

const US_FED_TAX: f32 = 0.2201; // this includes Fed Income Tax, Medicare, Social Secuirty
const CO_STATE_TAX: f32 = 0.042; // this includes State withholding and FLI withholding

const FOUR_0_1_K: f32 = 0.042;

fn main() -> Result<(), Box<dyn Error>> {
    
    print_hello();

    let ui = AppWindow::new()?;

    ui.on_calculate_salary({
        
        let ui_handle = ui.as_weak();

        move |string: SharedString| {
            let ui = ui_handle.unwrap();

            let num: f32 = string.trim().parse().unwrap();

            ui.set_monthly_check(calculate_monthly_check(num));
            ui.set_bi_weekly_check(calculate_biweekly_check(num));
            ui.set_bi_after_tax_check(calculate_after_tax_bi_check(num));
            ui.set_fed_tax(calculate_fed_tax(num));
            ui.set_state_tax(calculate_state_tax(num));
        }
    });

    ui.on_reset_calculation({
        
        let ui_handle = ui.as_weak();
        
        move || {
            let ui = ui_handle.unwrap();

            ui.set_monthly_check(0.0);
            ui.set_bi_weekly_check(0.0);
            ui.set_bi_after_tax_check(0.0);
            ui.set_fed_tax(0.0);
            ui.set_state_tax(0.0);

        }
    });

    ui.run()?;

    Ok(())
}

fn calculate_monthly_check(salary: f32) -> f32 {
    salary / 12.0
}

fn calculate_biweekly_check(salary: f32) -> f32 {
    salary / 26.0
}

fn calculate_after_tax_bi_check(salary: f32) -> f32 {
    let bi_sal = salary / 26.0;

    let fed_tax = bi_sal * US_FED_TAX;
    let state_tax = bi_sal * CO_STATE_TAX;

    bi_sal - fed_tax - state_tax
}

fn calculate_fed_tax(salary: f32) -> f32 {
    let bi_sal = salary / 26.0;
    let fed_tax = bi_sal * US_FED_TAX;
    fed_tax
}

fn calculate_state_tax(salary: f32) -> f32 {
    let bi_sal = salary / 26.0;
    let state_tax = bi_sal * CO_STATE_TAX;
    state_tax
}


