#![windows_subsystem = "windows"]

pub mod engine;
pub mod renderer;
pub mod window;

use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
    Win32::System::LibraryLoader::*,
};

use engine::state::GameState;
use renderer::{DioramaRenderer, TacticalRenderer};
use window::*;

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        let window_class = w!("TaskBar4XAlphaClass");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance.into(),
            lpszClassName: window_class,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        RegisterClassW(&wc);

        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);

        // Crear ventana inicial acoplada a la barra de tareas
        let hwnd = CreateWindowExW(
            WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            window_class,
            w!("TASK BAR 4X"),
            WS_POPUP | WS_VISIBLE,
            0,
            screen_h - APP_HEIGHT,
            screen_w,
            APP_HEIGHT,
            None,
            None,
            instance,
            None,
        )?;

        // Inicializar contexto del juego
        let ctx = WindowContext {
            hwnd,
            mode: WindowMode::DockedAppBar,
            state: GameState::new(),
            diorama: DioramaRenderer::new(),
            tactical: TacticalRenderer::new(),
            last_time: std::time::Instant::now(),
        };

        {
            let mut global_lock = GLOBAL_CTX.lock().unwrap();
            *global_lock = Some(Box::new(ctx));
        }

        // Registrar como AppBar en el borde inferior
        register_appbar(hwnd, true);

        // Registrar HotKey global Win + Alt + X
        let _ = RegisterHotKey(hwnd, HOTKEY_FS_ID, MOD_ALT | MOD_WIN, 0x58);

        // Bucle de mensajes reactivo con WaitMessage() para consumo 0.0% CPU
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
            let _ = WaitMessage();
        }

        let _ = UnregisterHotKey(hwnd, HOTKEY_FS_ID);
        register_appbar(hwnd, false);
    }

    Ok(())
}
