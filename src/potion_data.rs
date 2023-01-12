use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotionData {
    pub potion: Potion,
    pub potion_from_panel: PotionFromPanel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Potion {
    pub potion_name: String,
    pub custom_title: String,
    pub is_title_custom: bool,
    pub custom_description: String,
    pub custom_recipe_notes: String,
    pub use_custom_key_as_title: bool,
    pub effect_names: Vec<String>,
    pub icon_colors: Vec<IconColor>,
    pub are_colors_custom: Vec<bool>,
    pub bottle_name: String,
    pub icon_name: String,
    pub sticker_name: String,
    pub sticker_angle: i64,
    pub potion_base_name: String,
    pub used_components_names: Vec<String>,
    pub used_components_amounts: Vec<i64>,
    pub used_components_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotionFromPanel {
    pub potion_used_components: Vec<PotionUsedComponent>,
    pub recipe_marks: Vec<RecipeMark>,
    pub collected_potion_effects: Vec<String>,
    pub potion_skin_settings: PotionSkinSettings,
    pub serialized_path: SerializedPath,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotionUsedComponent {
    pub component_name: String,
    pub component_amount: i64,
    pub component_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeMark {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub float_value: f64,
    pub string_value: String,
    pub note: Note,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub note_index: i64,
    pub note_size: i64,
    pub note_digit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotionSkinSettings {
    pub current_bottle_name: String,
    pub current_sticker_name: String,
    pub current_sticker_angle: i64,
    pub current_icon_name: String,
    pub current_icon_color1: CurrentIconColor1,
    pub current_icon_color2: CurrentIconColor2,
    pub current_icon_color3: CurrentIconColor3,
    pub current_icon_color4: CurrentIconColor4,
    pub current_custom_title: String,
    pub is_current_title_custom: bool,
    pub current_description: String,
    pub current_recipe_notes: String,
    pub is_current_icon_custom: bool,
    #[serde(rename = "isCurrentIconColor1Custom")]
    pub is_current_icon_color1custom: bool,
    #[serde(rename = "isCurrentIconColor2Custom")]
    pub is_current_icon_color2custom: bool,
    #[serde(rename = "isCurrentIconColor3Custom")]
    pub is_current_icon_color3custom: bool,
    #[serde(rename = "isCurrentIconColor4Custom")]
    pub is_current_icon_color4custom: bool,
    pub colors_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentIconColor1 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentIconColor2 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentIconColor3 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentIconColor4 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedPath {
    pub fixed_path_points: Vec<FixedPathPoint>,
    pub indicator_rotation_value: f64,
    pub global_cross_parameters: GlobalCrossParameters,
    pub global_spiral_parameters: GlobalSpiralParameters,
    pub deleted_graphics_segments: f64,
    pub segment_length_to_delete_from_end_physics: f64,
    pub segment_length_to_delete_from_end_graphics: f64,
    pub segment_length_to_delete_from_end_dots: f64,
    pub segment_length_to_delete_physics: f64,
    pub segment_length_to_delete_graphics: f64,
    pub indicator_target_position: IndicatorTargetPosition,
    pub path_position: PathPosition,
    pub indicator_position: IndicatorPosition,
    pub follow_button_target_object_position: FollowButtonTargetObjectPosition,
    pub health: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedPathPoint {
    pub physics_points: Vec<PhysicsPoint>,
    pub graphics_points: Vec<GraphicsPoint>,
    pub dots_points: Vec<Value>,
    pub path_end_parameters: PathEndParameters,
    pub path_start_parameters: PathStartParameters,
    pub is_teleportation_hint: bool,
    pub is_available_for_teleportation: bool,
    pub graphics_path_length_on_teleportation_animation_start: f64,
    pub moving_along_path_status: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicsPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphicsPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathEndParameters {
    pub sprite_index: i64,
    pub rotation: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathStartParameters {
    pub sprite_index: i64,
    pub rotation: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalCrossParameters {
    pub sprite_index: i64,
    pub rotation: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSpiralParameters {
    pub sprite_index: i64,
    pub rotation: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndicatorTargetPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndicatorPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowButtonTargetObjectPosition {
    pub x: f64,
    pub y: f64,
}
