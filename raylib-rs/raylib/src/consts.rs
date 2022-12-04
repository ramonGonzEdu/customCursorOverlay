//! Various constant enums to use with raylib
pub use ffi::{
    BlendMode, CameraMode, CameraProjection, ConfigFlags, CubemapLayout, GamepadAxis,
    GamepadButton, Gestures, KeyboardKey, MaterialMapIndex, MouseButton, NPatchLayout, PixelFormat,
    ShaderLocationIndex, ShaderUniformDataType, TextureFilter, TextureWrap, TraceLogLevel, DEG2RAD,
};

pub use crate::ffi;
// TODO Fix when rlgl bindings are in
pub const MAX_MATERIAL_MAPS: u32 = 12;
pub const MAX_SHADER_LOCATIONS: u32 = 32;
pub use ffi::{
    guiIconName, GuiCheckBoxProperty, GuiColorPickerProperty, GuiComboBoxProperty, GuiControl,
    GuiControlProperty, GuiControlState, GuiDefaultProperty, GuiDropdownBoxProperty,
    GuiListViewProperty, GuiProgressBarProperty, GuiScrollBarProperty, GuiScrollBarSide,
    GuiSliderProperty, GuiSpinnerProperty, GuiTextAlignment, GuiTextBoxProperty, GuiToggleProperty,
    MouseCursor, PI, RAD2DEG,
};
