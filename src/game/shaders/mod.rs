use super::*;

pub fn ui_shader() -> Shader {
    const VSCODE: &[u8] = include_bytes!("ui.vert");
    const FSCODE: &[u8] = include_bytes!("ui.frag");
    unsafe { Shader::new(VSCODE, FSCODE) }
}

pub fn game_shader() -> Shader {
    const VSCODE: &[u8] = include_bytes!("game.vert");
    const FSCODE: &[u8] = include_bytes!("game.frag");
    unsafe { Shader::new(VSCODE, FSCODE) }
}
