use mlua::prelude::*;

use std::fs;

// add from LuaError for LumaError

async fn run() -> Result<()> {
    let lua = Lua::new();
    // let map_table = lua.create_table()?;
    // map_table.set(1, "one")?;
    // map_table.set("two", 2)?;
    // lua.globals().set("map_table", map_table)?;

    let globals = lua.globals();
    globals.set("luma", LUMA.read().unwrap());

    lua.set_hook(HookTriggers::EVERY_LINE, |_lua, debug| {
        println!("line {}", debug.curr_line());
        Ok(())
    });

    lua.load(
        r#"
    local x = 2 + 3
    local y = x * 63
    local z = string.len(x..", "..y)
"#,
    )
    .exec();
    let greet = lua.create_function(|_, name: String| {
        println!("Hello, {}!", name);
        Ok(())
    });

    let my_table: Table = lua
        .load(
            r#"
    {
        [1] = 4,
        [2] = 5,
        [4] = 7,
        key = 2
    }
"#,
        )
        .eval()?;

    let table1 = lua.create_table()?;
    table1.set(1, "value")?;

    let print_person = lua.create_function(|_, (name, age): (String, u8)| {
        println!("{} is {} years old!", name, age);
        Ok(())
    });

    Ok(())
}

fn factorial(n: u64, lua: &Lua) -> String {
    let script = fs::read_to_string("./lua/factorial.lua").unwrap();
    let global = lua.globals();
    _ = global.set("arg", vec![n]);
    let res = lua.load(&script).eval().unwrap();

    res
}
