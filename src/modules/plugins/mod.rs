use std::{fs, path::Path};

use mlua::prelude::*;

use crate::core::stores::global_store::global_store;

pub static mut MOUSE_ENABLED: bool = true;

//pub fn read_extensions_folder() {
//    let extension_path = Path::new("extensions/");
//    let paths = fs::read_dir(extension_path).expect("Don't found folder");
//
//    for path in paths.flatten() {
//        // path — это Result, так как при чтении файла может возникнуть ошибка
//        if let Ok(entry) = path {
//            let file_path = entry.path();
//
//            // Проверяем, что это файл и у него расширение .lua
//            if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("lua")
//            {
//                println!("Нашел плагин: {:?}", file_path);
//
//                // Вот здесь уже можно читать содержимое конкретного файла
//                let content = fs::read_to_string(&file_path).expect("Ошибка чтения файла");
//                // Дальше прокидываешь content в свой Lua-движок
//            }
//        }
//    }
//}

pub fn run_test_plugin() -> LuaResult<()> {
    let lua = Lua::new();
    change_font_size();

    let config_path = std::path::Path::new("test_plugin.lua");
    let script = std::fs::read_to_string(config_path).expect("Не нашел файл плагина");

    let globals = lua.globals();

    let disable_mouse = lua.create_function(|_, ()| {
        println!("Плагин запросил отключение мыши!");
        unsafe {
            MOUSE_ENABLED = false;
        }
        Ok(())
    })?;

    globals.set("disable_mouse", disable_mouse)?;
    lua.load(&script).exec()?;

    Ok(())
}

pub fn change_font_size() {
    let current_font_size = global_store().get_font_size();
    println!("Current font size {:?}", current_font_size)
}

// TODO: поменять размер шрифта через колесико мыши
// 1. написать функцию change_font_size - done
// 2. берет текущее значение шрифта (его размер) - done
// 3. прибавляет или уменьшает (от 8 до 72)
// 4. привязываем эту функцию к событию колесика мыши
// and need
// Добавить поле font_size: f32 в твою главную структуру.
//
// В методе update прочитать ctx.input(|i| i.scroll_delta.y).
//
// Вызвать свою функцию change_font_size, если дельта не равна нулю.
//
// Внутри функции использовать .clamp(8.0, 72.0).
//
// Вызвать обновление стилей egui, чтобы изменения применились.
