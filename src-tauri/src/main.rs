// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::sleep, time::Duration};

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set_always_on_top(window: tauri::Window) {
  window.set_always_on_top(true);
}
#[tauri::command]
fn show(window: tauri::Window) {
    std::thread::spawn(move ||{
        //std::thread::sleep(Duration::from_secs(1));
        window.show().unwrap();
    });
 
}

fn main() {
    tauri::Builder::default()
   
    .setup(|app| {
        let window = app.get_window("main").unwrap();
        let window_clone = window.clone();
        std::thread::spawn(move || {
            sleep(Duration::from_secs(3));
            window_clone.eval(r#"
            //sel trans to language
            //let sel='body > c-wiz > div > div > c-wiz > div > c-wiz > div > div:nth-child(1) > c-wiz > div > c-wiz > div:nth-child(5) > button > div';
            //let collapse_sel = 'body > c-wiz > div > div > c-wiz > div > c-wiz > div > div:nth-child(1) > c-wiz > div > c-wiz > div:nth-child(5) > button > div';
            let selchinese = 'body > c-wiz > div > div > c-wiz > div > c-wiz > div > div:nth-child(1) > c-wiz > div:nth-child(2) > c-wiz > div > div > div > div > div:nth-child(2) > span:nth-child(20) > div > div';

            function click_element(selector){
                var element = document.querySelector(selector);
                element.click();
            }
            click_element(selchinese);
            /*
            setTimeout(()=>{
                click_element(sel);
                setTimeout(()=>{
                    click_element(selchinese);
                        setTimeout(()=>{
                            click_element(collapse_sel);
                        },600);
                },100);
            },300);
            */
            //hide elements
            function hidden(selector) {
                var element = document.querySelector(selector);
                element.style.display = 'none';
            }
            function click_element(selector){
                var element = document.querySelector(selector);
                element.click();
            }
            hidden('#gb');
            hidden('body > c-wiz > div > div');
            hidden('body > c-wiz > div > div > c-wiz > nav');
            //hidden('body > c-wiz > div > div > c-wiz > div > c-wiz > div > div');

            "#).unwrap();
            sleep(Duration::from_secs_f64(1.5));
            show(window.clone());
        });
        Ok(())
         })  
        .invoke_handler(tauri::generate_handler![set_always_on_top,show])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
