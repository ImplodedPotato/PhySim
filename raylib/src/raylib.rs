#![allow(dead_code)]

use std::ops::BitOr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Vector2 {
    pub unsafe fn zero() -> Vector2 {
        vector_2_zero()
    }
    pub unsafe fn one() -> Vector2 {
        vector_2_one()
    }
    pub unsafe fn new_from(num: f32) -> Vector2 {
        Vector2{ x: num, y: num }
    }

    pub fn pythagorean(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
    pub fn add(self, vec: Vector2) -> Vector2 {
        unsafe { vector_2_add(self, vec) }
    }
    pub fn addeq(&mut self, vec: Vector2) {
        unsafe { *self = vector_2_add(*self, vec); }
    }
    pub fn sub(self, vec: Vector2) -> Vector2 {
        unsafe { vector_2_subtract(self, vec) }
    }
    pub fn subeq(&mut self, vec: Vector2) {
        unsafe { *self = vector_2_subtract(*self, vec); }
    }
    pub fn sub_value(self, value: f32) -> Vector2 {
        unsafe { vector_2_subtract_value(self, value) }
    }
    pub fn subeq_value(&mut self, value: f32) {
         unsafe { *self = vector_2_subtract_value(*self, value) }
    }
    pub fn mult(self, vec: Vector2) -> Vector2 {
        unsafe { vector_2_multiply(self, vec) }
    }
    pub fn multeq(&mut self, vec: Vector2) {
        unsafe { *self = vector_2_multiply(*self, vec); }
    }
    pub fn mult_value(self, value: f32) -> Vector2 {
        Vector2{ x: self.x * value, y: self.y * value }
    }
    pub fn multeq_value(&mut self, value: f32) {
         (*self).x *= value;
         (*self).y *= value;
    }
    pub fn div(self, vec: Vector2) -> Vector2 {
        unsafe { vector_2_divide(self, vec) }
    }
    pub fn diveq(&mut self, vec: Vector2) {
        unsafe { *self = vector_2_divide(*self, vec); }
    }
    pub fn div_value(self, value: f32) -> Vector2 {
        Vector2{ x: self.x / value, y: self.y / value }
    }
    pub fn diveq_value(&mut self, value: f32) {
         (*self).x /= value;
         (*self).y /= value;
    }
    pub fn normalize(self) -> Vector2 {
        unsafe { vector_2_normalize(self) }
    }
    pub fn dot(self, other: Vector2) -> f32 {
        unsafe { vector_2_dot_product(self, other) }
    }
}


pub unsafe fn init_window(width: i32, height: i32, title: &str) {
    init_window_internal(width, height, (title.to_string() + "\0").as_ptr() as *const i8);
}

pub unsafe fn get_screen_dimensions() -> Vector2 {
    Vector2{ x: get_screen_width() as f32, y: get_screen_height() as f32 }
}

extern "C" {
// rcore
    // Window-related functions
    #[link_name="InitWindow"]
    fn init_window_internal(width: i32, height: i32, title: *const i8);
    #[link_name="CloseWindow"]
    pub fn close_window();
    #[link_name="WindowShouldClose"]
    pub fn window_should_close() -> bool;
    #[link_name="IsWindowResized"]
    pub fn is_window_resized() -> bool;
    #[link_name="IsWindowFocused"]
    pub fn is_window_focused() -> bool;
    #[link_name="SetWindowPosition"]
    pub fn set_window_position(x: i32, y: i32);
    #[link_name="SetWindowOpacity"]
    pub fn set_window_opacity(opacity: f32);
    #[link_name="GetScreenWidth"]
    pub fn get_screen_width() -> i32;
    #[link_name="GetScreenHeight"]
    pub fn get_screen_height() -> i32;

    // Drawing-related functions
    #[link_name="ClearBackground"]
    pub fn clear_background(color: Color);
    #[link_name="BeginDrawing"]
    pub fn begin_drawing();
    #[link_name="EndDrawing"]
    pub fn end_drawing();

    // Timing-related functions
    #[link_name="SetTargetFPS"]
    pub fn set_target_fps(fps: i32);
    #[link_name="GetFrameTime"]
    pub fn get_frame_time() -> f32;
    #[link_name="GetFPS"]
    pub fn get_fps() -> f64;

    // Misc. functions
    #[link_name="SetConfigFlags"]
    pub fn set_config_flags(flags: u32);

    // Input-related functions: keyboard
    #[link_name="IsKeyPressed"]
    pub fn is_key_pressed(key: KeyboardKey) -> bool;
    #[link_name="IsKeyDown"]
    pub fn is_key_down(key: KeyboardKey) -> bool;

    // Input-related functions: mouse
    #[link_name="GetMousePosition"]
    pub fn get_mouse_position() -> Vector2;
    #[link_name="IsMouseButtonPressed"]
    pub fn is_mouse_button_pressed(button: MouseButton) -> bool;

// rshapes
    // Basic shapes drawing functions
    #[link_name="DrawCircle"]
    pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color);
    #[link_name="DrawCircleV"]
    pub fn draw_circle_v(center: Vector2, radius: f32, color: Color);
    #[link_name="DrawCircleLines"]
    pub fn draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color);
    #[link_name="DrawCircleLinesV"]
    pub fn draw_circle_lines_v(center: Vector2, radius: f32, color: Color);
    #[link_name="DrawRectangle"]
    pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
    #[link_name="DrawRectangleV"]
    pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color);
    #[link_name="DrawRectangleRec"]
    pub fn draw_rectangle_rec(rec: Rectangle, color: Color);
    #[link_name="DrawRectangleLines"]
    pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
    #[link_name="DrawRectangleLinesEx"]
    pub fn draw_rectangle_lines_ex(rec: Rectangle, line_thick: f32, color: Color);

    // Basic shapes collision detection functions
    #[link_name="CheckCollisionCircles"]
    pub fn check_collision_circles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32) -> bool;
    #[link_name="CheckCollisionCircleLine"]
    pub fn check_collision_circle_line(center: Vector2, radius: f32, p1: Vector2, p2: Vector2);
    #[link_name="CheckCollisionPointRec"]
    pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool;

// rtextures
    // Color/pixel related functions
    #[link_name="GetColor"]
    pub fn get_color(hexValue: u32) -> Color;

// rtext
    // Text drawing functions
    #[link_name="DrawFPS"]
    pub fn draw_fps(pos_x: i32, pos_y: i32);
    #[link_name="DrawText"]
    pub fn draw_text(text: *const i8, pos_x: i32, pos_y: i32, font_size: i32, color: Color);

    // Text font info functions
    #[link_name="MeasureText"]
    pub fn measure_text(text: *const i8, font_size: i32) -> i32;

    // Text strings management functions (no UTF-8 strings, only byte chars)
    #[link_name="TextFormat"]
    pub fn text_format(text: *const i8, ...) -> *const i8;

// rmath
    // Utils math
    #[link_name="FloatEquals"]
    pub fn float_equals(x: f32, y: f32) -> i32;

    // Vector2 math
    #[link_name="Vector2Zero"]
    pub fn vector_2_zero() -> Vector2;
    #[link_name="Vector2One"]
        pub fn vector_2_one() -> Vector2;
    #[link_name="Vector2Add"]
    pub fn vector_2_add(v1: Vector2, v2: Vector2) -> Vector2;
    #[link_name="Vector2AddValue"]
    pub fn vector_2_add_value(v1: Vector2, add: f32) -> Vector2;
    #[link_name="Vector2Subtract"]
    pub fn vector_2_subtract(v1: Vector2, v2: Vector2) -> Vector2;
    #[link_name="Vector2SubtractValue"]
    pub fn vector_2_subtract_value(v1: Vector2, sub: f32) -> Vector2;
    #[link_name="Vector2DotProduct"]
    pub fn vector_2_dot_product(v1: Vector2, v2: Vector2) -> f32;
    #[link_name="Vector2Multiply"]
    pub fn vector_2_multiply(v1: Vector2, v2: Vector2) -> Vector2;
    #[link_name="Vector2Normalize"]
    pub fn vector_2_normalize(v: Vector2) -> Vector2;
    #[link_name="Vector2Divide"]
    pub fn vector_2_divide(v1: Vector2, v2: Vector2) -> Vector2;
}

#[repr(u32)]
pub enum ConfigFlags {
    FlagVsyncHint              = 0x00000040,   // Set to try enabling V-Sync on GPU
    FlagFullscreenMode         = 0x00000002,   // Set to run program in fullscreen
    FlagWindowResizable        = 0x00000004,   // Set to allow resizable window
    FlagWindowUndecorated      = 0x00000008,   // Set to disable window decoration (frame and buttons)
    FlagWindowHidden           = 0x00000080,   // Set to hide window
    FlagWindowMinimized        = 0x00000200,   // Set to minimize window (iconify)
    FlagWindowMaximized        = 0x00000400,   // Set to maximize window (expanded to monitor)
    FlagWindowUnfocused        = 0x00000800,   // Set to window non focused
    FlagWindowTopmost          = 0x00001000,   // Set to window always on top
    FlagWindowAlwaysRun        = 0x00000100,   // Set to allow windows running while minimized
    FlagWindowTransparent      = 0x00000010,   // Set to allow transparent framebuffer
    FlagWindowHighdpi          = 0x00002000,   // Set to support HighDPI
    FlagWindowMousePassthrough = 0x00004000, // Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
    FlagBorderlessWindowedMode = 0x00008000, // Set to run program in borderless windowed mode
    FlagMsaa4xHint             = 0x00000020,   // Set to try enabling MSAA 4X
    FlagInterlacedHint         = 0x00010000    // Set to try enabling interlaced video format (for V3D)
}

impl BitOr for ConfigFlags {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output { self as u32 | rhs as u32 }
}

#[repr(C)]
pub enum KeyboardKey {
    KeyNull            = 0,        // Key: NULL, used for no key pressed
    // Alphanumeric keys
    KeyApostrophe      = 39,       // Key: '
    KeyComma           = 44,       // Key: ,
    KeyMinus           = 45,       // Key: -
    KeyPeriod          = 46,       // Key: .
    KeySlash           = 47,       // Key: /
    KeyZero            = 48,       // Key: 0
    KeyOne             = 49,       // Key: 1
    KeyTwo             = 50,       // Key: 2
    KeyThree           = 51,       // Key: 3
    KeyFour            = 52,       // Key: 4
    KeyFive            = 53,       // Key: 5
    KeySix             = 54,       // Key: 6
    KeySeven           = 55,       // Key: 7
    KeyEight           = 56,       // Key: 8
    KeyNine            = 57,       // Key: 9
    KeySemicolon       = 59,       // Key: ;
    KeyEqual           = 61,       // Key: =
    KeyA               = 65,       // Key: A | a
    KeyB               = 66,       // Key: B | b
    KeyC               = 67,       // Key: C | c
    KeyD               = 68,       // Key: D | d
    KeyE               = 69,       // Key: E | e
    KeyF               = 70,       // Key: F | f
    KeyG               = 71,       // Key: G | g
    KeyH               = 72,       // Key: H | h
    KeyI               = 73,       // Key: I | i
    KeyJ               = 74,       // Key: J | j
    KeyK               = 75,       // Key: K | k
    KeyL               = 76,       // Key: L | l
    KeyM               = 77,       // Key: M | m
    KeyN               = 78,       // Key: N | n
    KeyO               = 79,       // Key: O | o
    KeyP               = 80,       // Key: P | p
    KeyQ               = 81,       // Key: Q | q
    KeyR               = 82,       // Key: R | r
    KeyS               = 83,       // Key: S | s
    KeyT               = 84,       // Key: T | t
    KeyU               = 85,       // Key: U | u
    KeyV               = 86,       // Key: V | v
    KeyW               = 87,       // Key: W | w
    KeyX               = 88,       // Key: X | x
    KeyY               = 89,       // Key: Y | y
    KeyZ               = 90,       // Key: Z | z
    KeyLeftBracket     = 91,       // Key: [
    KeyBackslash       = 92,       // Key: '\'
    KeyRightBracket    = 93,       // Key: ]
    KeyGrave           = 96,       // Key: `
    // Function keys
    KeySpace           = 32,       // Key: Space
    KeyEscape          = 256,      // Key: Esc
    KeyEnter           = 257,      // Key: Enter
    KeyTab             = 258,      // Key: Tab
    KeyBackspace       = 259,      // Key: Backspace
    KeyInsert          = 260,      // Key: Ins
    KeyDelete          = 261,      // Key: Del
    KeyRight           = 262,      // Key: Cursor right
    KeyLeft            = 263,      // Key: Cursor left
    KeyDown            = 264,      // Key: Cursor down
    KeyUp              = 265,      // Key: Cursor up
    KeyPageUp          = 266,      // Key: Page up
    KeyPageDown        = 267,      // Key: Page down
    KeyHome            = 268,      // Key: Home
    KeyEnd             = 269,      // Key: End
    KeyCapsLock        = 280,      // Key: Caps lock
    KeyScrollLock      = 281,      // Key: Scroll down
    KeyNumLock         = 282,      // Key: Num lock
    KeyPrintScreen     = 283,      // Key: Print screen
    KeyPause           = 284,      // Key: Pause
    KeyF1              = 290,      // Key: F1
    KeyF2              = 291,      // Key: F2
    KeyF3              = 292,      // Key: F3
    KeyF4              = 293,      // Key: F4
    KeyF5              = 294,      // Key: F5
    KeyF6              = 295,      // Key: F6
    KeyF7              = 296,      // Key: F7
    KeyF8              = 297,      // Key: F8
    KeyF9              = 298,      // Key: F9
    KeyF10             = 299,      // Key: F10
    KeyF11             = 300,      // Key: F11
    KeyF12             = 301,      // Key: F12
    KeyLeftShift       = 340,      // Key: Shift left
    KeyLeftControl     = 341,      // Key: Control left
    KeyLeftAlt         = 342,      // Key: Alt left
    KeyLeftSuper       = 343,      // Key: Super left
    KeyRightShift      = 344,      // Key: Shift right
    KeyRightControl    = 345,      // Key: Control right
    KeyRightAlt        = 346,      // Key: Alt right
    KeyRightSuper      = 347,      // Key: Super right
    KeyKbMenu          = 348,      // Key: KB menu
    // Keypad keys
    KeyKp0             = 320,      // Key: Keypad 0
    KeyKp1             = 321,      // Key: Keypad 1
    KeyKp2             = 322,      // Key: Keypad 2
    KeyKp3             = 323,      // Key: Keypad 3
    KeyKp4             = 324,      // Key: Keypad 4
    KeyKp5             = 325,      // Key: Keypad 5
    KeyKp6             = 326,      // Key: Keypad 6
    KeyKp7             = 327,      // Key: Keypad 7
    KeyKp8             = 328,      // Key: Keypad 8
    KeyKp9             = 329,      // Key: Keypad 9
    KeyKpDecimal       = 330,      // Key: Keypad .
    KeyKpDivide        = 331,      // Key: Keypad /
    KeyKpMultiply      = 332,      // Key: Keypad *
    KeyKpSubtract      = 333,      // Key: Keypad -
    KeyKpAdd           = 334,      // Key: Keypad +
    KeyKpEnter         = 335,      // Key: Keypad Enter
    KeyKpEqual         = 336,      // Key: Keypad =
    // Android key buttons
    KeyBack            = 4,        // Key: Android back button
    KeyMenu            = 5,        // Key: Android menu button
    KeyVolumeUp        = 24,       // Key: Android volume up button
    KeyVolumeDown      = 25        // Key: Android volume down button
}

#[repr(C)]
pub enum MouseButton {
    MouseButtonLeft    = 0,       // Mouse button left
    MouseButtonRight   = 1,       // Mouse button right
    MouseButtonMiddle  = 2,       // Mouse button middle (pressed wheel)
    MouseButtonSide    = 3,       // Mouse button side (advanced mouse device)
    MouseButtonExtra   = 4,       // Mouse button extra (advanced mouse device)
    MouseButtonForward = 5,       // Mouse button forward (advanced mouse device)
    MouseButtonBack    = 6,       // Mouse button back (advanced mouse device)
}


// Colors
pub const LIGHTGRAY  : Color = Color{ r: 200, g: 200, b: 200, a: 255 } ;  // Light Gray
pub const GRAY       : Color = Color{ r: 130, g: 130, b: 130, a: 255 } ;  // Gray
pub const DARKGRAY   : Color = Color{ r: 80, g: 80, b: 80, a: 255 }    ;  // Dark Gray
pub const YELLOW     : Color = Color{ r: 253, g: 249, b: 0, a: 255 }   ;  // Yellow
pub const GOLD       : Color = Color{ r: 255, g: 203, b: 0, a: 255 }   ;  // Gold
pub const ORANGE     : Color = Color{ r: 255, g: 161, b: 0, a: 255 }   ;  // Orange
pub const PINK       : Color = Color{ r: 255, g: 109, b: 194, a: 255 } ;  // Pink
pub const RED        : Color = Color{ r: 230, g: 41, b: 55, a: 255 }   ;  // Red
pub const MAROON     : Color = Color{ r: 190, g: 33, b: 55, a: 255 }   ;  // Maroon
pub const GREEN      : Color = Color{ r: 0, g: 228, b: 48, a: 255 }    ;  // Green
pub const LIME       : Color = Color{ r: 0, g: 158, b: 47, a: 255 }    ;  // Lime
pub const DARKGREEN  : Color = Color{ r: 0, g: 117, b: 44, a: 255 }    ;  // Dark Green
pub const SKYBLUE    : Color = Color{ r: 102, g: 191, b: 255, a: 255 } ;  // Sky Blue
pub const BLUE       : Color = Color{ r: 0, g: 121, b: 241, a: 255 }   ;  // Blue
pub const DARKBLUE   : Color = Color{ r: 0, g: 82, b: 172, a: 255 }    ;  // Dark Blue
pub const PURPLE     : Color = Color{ r: 200, g: 122, b: 255, a: 255 } ;  // Purple
pub const VIOLET     : Color = Color{ r: 135, g: 60, b: 190, a: 255 }  ;  // Violet
pub const DARKPURPLE : Color = Color{ r: 112, g: 31, b: 126, a: 255 }  ;  // Dark Purple
pub const BEIGE      : Color = Color{ r: 211, g: 176, b: 131, a: 255 } ;  // Beige
pub const BROWN      : Color = Color{ r: 127, g: 106, b: 79, a: 255 }  ;  // Brown
pub const DARKBROWN  : Color = Color{ r: 76, g: 63, b: 47, a: 255 }    ;  // Dark Brown

pub const WHITE      : Color = Color{ r: 255, g: 255, b: 255, a: 255 } ;  // White
pub const BLACK      : Color = Color{ r: 0, g: 0, b: 0, a: 255 }       ;  // Black
pub const BLANK      : Color = Color{ r: 0, g: 0, b: 0, a: 0 }         ;  // Blank (Transparent)
pub const MAGENTA    : Color = Color{ r: 255, g: 0, b: 255, a: 255 }   ;  // Magenta
pub const RAYWHITE   : Color = Color{ r: 245, g: 245, b: 245, a: 255 } ;  // My own White (raylib logo)
