#[derive(Debug)]
pub struct InputManager {
    pressed_keys: Vec<KeyCode>,
    just_pressed_keys: Vec<KeyCode>,
    pressed_mouse_buttons: Vec<MouseButton>,
    just_pressed_mouse_buttons: Vec<MouseButton>,
    mouse_position: (f32, f32),
    mouse_delta: (f32, f32),
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            pressed_keys: vec![],
            just_pressed_keys: vec![],
            pressed_mouse_buttons: vec![],
            just_pressed_mouse_buttons: vec![],
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn update(&mut self, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::KeyboardInput { input, .. } => {
                self.handle_keyboard_input(input)
            }
            winit::event::WindowEvent::MouseInput { button, state, .. } => {
                self.handle_mouse_input(button, state)
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => self.handle_cursor(position),
            winit::event::WindowEvent::MouseWheel { delta, .. } => self.handle_mouse_wheel(delta),
            _ => {}
        }
    }

    // is neaded to be called after all main event are cleared and the application logic for this
    // update is completed
    pub fn clear(&mut self) {
        self.just_pressed_keys.clear();
        self.just_pressed_mouse_buttons.clear();
        self.mouse_delta = (0.0, 0.0)
    }

    fn handle_keyboard_input(&mut self, input: winit::event::KeyboardInput) {
        if input.virtual_keycode.is_none() {
            return;
        }
        if input.state == winit::event::ElementState::Pressed {
            let input: KeyCode = input.virtual_keycode.unwrap().into();

            if self.pressed_keys.contains(&input) {
                return;
            } else {
                self.just_pressed_keys.push(input.clone());
                self.pressed_keys.push(input);
            }
        } else {
            self.pressed_keys.retain(|key| {
                // thats weird
                key != &<winit::event::VirtualKeyCode as Into<KeyCode>>::into(
                    input.virtual_keycode.unwrap(),
                )
            })
        }
    }

    fn handle_mouse_input(
        &mut self,
        input: winit::event::MouseButton,
        state: winit::event::ElementState,
    ) {
        if state == winit::event::ElementState::Pressed {
            let input: MouseButton = input.into();

            if self.pressed_mouse_buttons.contains(&input) {
                return;
            } else {
                self.just_pressed_mouse_buttons.push(input.clone());
                self.pressed_mouse_buttons.push(input);
            }
        } else {
            self.pressed_mouse_buttons.retain(|key| {
                // thats weird
                key != &<winit::event::MouseButton as Into<MouseButton>>::into(input)
            })
        }
    }

    fn handle_cursor(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        self.mouse_position = (position.x as f32, position.y as f32)
    }

    fn handle_mouse_wheel(&mut self, delta: winit::event::MouseScrollDelta) {
        if let winit::event::MouseScrollDelta::LineDelta(x, y) = delta {
            self.mouse_delta = (x, y)
        }
    }

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    pub fn is_key_just_pressed(&self, key_code: KeyCode) -> bool {
        self.just_pressed_keys.contains(&key_code)
    }

    pub fn is_key_released(&self, key_code: KeyCode) -> bool {
        !self.is_key_pressed(key_code)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mouse_buttons.contains(&button)
    }

    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        !self.is_mouse_button_pressed(button)
    }

    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Enter,
    Space,
    Tab,
    Backspace,
    Home,
    Escape,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    Up,
    Down,
    Left,
    Right,
    Unimplemented,
}

impl From<winit::event::VirtualKeyCode> for KeyCode {
    fn from(value: winit::event::VirtualKeyCode) -> Self {
        match value {
            winit::event::VirtualKeyCode::A => Self::A,
            winit::event::VirtualKeyCode::B => Self::B,
            winit::event::VirtualKeyCode::C => Self::C,
            winit::event::VirtualKeyCode::D => Self::D,
            winit::event::VirtualKeyCode::E => Self::E,
            winit::event::VirtualKeyCode::F => Self::F,
            winit::event::VirtualKeyCode::G => Self::G,
            winit::event::VirtualKeyCode::H => Self::H,
            winit::event::VirtualKeyCode::I => Self::I,
            winit::event::VirtualKeyCode::J => Self::J,
            winit::event::VirtualKeyCode::K => Self::K,
            winit::event::VirtualKeyCode::L => Self::L,
            winit::event::VirtualKeyCode::M => Self::M,
            winit::event::VirtualKeyCode::N => Self::N,
            winit::event::VirtualKeyCode::O => Self::O,
            winit::event::VirtualKeyCode::P => Self::P,
            winit::event::VirtualKeyCode::Q => Self::Q,
            winit::event::VirtualKeyCode::R => Self::R,
            winit::event::VirtualKeyCode::S => Self::S,
            winit::event::VirtualKeyCode::T => Self::T,
            winit::event::VirtualKeyCode::U => Self::U,
            winit::event::VirtualKeyCode::V => Self::V,
            winit::event::VirtualKeyCode::W => Self::W,
            winit::event::VirtualKeyCode::X => Self::X,
            winit::event::VirtualKeyCode::Y => Self::Y,
            winit::event::VirtualKeyCode::Z => Self::Z,

            winit::event::VirtualKeyCode::Key0 => Self::Zero,
            winit::event::VirtualKeyCode::Key1 => Self::One,
            winit::event::VirtualKeyCode::Key2 => Self::Two,
            winit::event::VirtualKeyCode::Key3 => Self::Three,
            winit::event::VirtualKeyCode::Key4 => Self::Four,
            winit::event::VirtualKeyCode::Key5 => Self::Five,
            winit::event::VirtualKeyCode::Key6 => Self::Six,
            winit::event::VirtualKeyCode::Key7 => Self::Seven,
            winit::event::VirtualKeyCode::Key8 => Self::Eight,
            winit::event::VirtualKeyCode::Key9 => Self::Nine,

            winit::event::VirtualKeyCode::F1 => Self::F1,
            winit::event::VirtualKeyCode::F2 => Self::F2,
            winit::event::VirtualKeyCode::F3 => Self::F3,
            winit::event::VirtualKeyCode::F4 => Self::F4,
            winit::event::VirtualKeyCode::F5 => Self::F5,
            winit::event::VirtualKeyCode::F6 => Self::F6,
            winit::event::VirtualKeyCode::F7 => Self::F7,
            winit::event::VirtualKeyCode::F8 => Self::F8,
            winit::event::VirtualKeyCode::F9 => Self::F9,
            winit::event::VirtualKeyCode::F10 => Self::F10,
            winit::event::VirtualKeyCode::F11 => Self::F11,
            winit::event::VirtualKeyCode::F12 => Self::F12,

            winit::event::VirtualKeyCode::Return => Self::Enter,
            winit::event::VirtualKeyCode::Space => Self::Space,
            winit::event::VirtualKeyCode::Tab => Self::Tab,
            winit::event::VirtualKeyCode::Back => Self::Backspace,
            winit::event::VirtualKeyCode::Home => Self::Home,
            winit::event::VirtualKeyCode::Escape => Self::Escape,
            winit::event::VirtualKeyCode::LShift => Self::LShift,
            winit::event::VirtualKeyCode::RShift => Self::RShift,
            winit::event::VirtualKeyCode::LControl => Self::LCtrl,
            winit::event::VirtualKeyCode::RControl => Self::RCtrl,
            winit::event::VirtualKeyCode::LAlt => Self::LAlt,
            winit::event::VirtualKeyCode::RAlt => Self::RAlt,
            winit::event::VirtualKeyCode::Up => Self::Up,
            winit::event::VirtualKeyCode::Down => Self::Down,
            winit::event::VirtualKeyCode::Left => Self::Left,
            winit::event::VirtualKeyCode::Right => Self::Right,
            _ => KeyCode::Unimplemented,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    MouseWheel,
    Unimplemented,
}

impl From<winit::event::MouseButton> for MouseButton {
    fn from(value: winit::event::MouseButton) -> Self {
        match value {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::MouseWheel,
            _ => MouseButton::Unimplemented,
        }
    }
}
