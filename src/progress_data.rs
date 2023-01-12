use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressData {
    pub systems_data: Vec<SystemsDaum>,
    pub current_level: i64,
    pub legendary_substances_brewed_amount: i64,
    pub is_animating: bool,
    pub slots: Slots,
    pub unlocked_legendary_recipes: Vec<String>,
    pub legendary_recipe_bookmarks: Vec<LegendaryRecipeBookmark>,
    pub legendary_recipes: Vec<String>,
    pub brewed_legendary_recipes: Vec<String>,
    pub read_unlocked_legendary_recipes: Vec<String>,
    pub current_legendary_recipe_index: i64,
    pub with_spawned_product: bool,
    pub unlocked_legendary_recipes_notification_count: i64,
    pub day: i64,
    pub can_take_ingredients: bool,
    pub heat: f64,
    pub previous_heat: f64,
    pub coals_heat: CoalsHeat,
    pub saved_colors: Vec<SavedColor>,
    pub colors_history: Vec<ColorsHistory>,
    pub current_space_index_for_color: i64,
    pub serialized_current_potion: SerializedCurrentPotion,
    pub serialized_default_bottle_settings: SerializedDefaultBottleSettings,
    pub potion_changed_after_saving_recipe: bool,
    pub previous_potion_recipe: PreviousPotionRecipe,
    pub was_developer_mode_on: bool,
    pub dialogues_global_condition_properties: Vec<Value>,
    pub dialogues_local_condition_properties: Vec<Value>,
    pub current_dialogue_path: Vec<String>,
    pub read_dialogue_nodes: Vec<String>,
    pub item_cost_multipliers: Vec<Value>,
    pub next_dialogue_state: i64,
    pub wind_speed: f64,
    pub total_day_npc_count: i64,
    pub served_npc_count: i64,
    pub current_mortar_stains: Vec<Value>,
    pub mortar_stains_to_remove: Vec<Value>,
    pub current_pestle_stains: Vec<Value>,
    pub pestle_stains_to_remove: Vec<Value>,
    pub experience: Experience,
    pub experience_bonus_map_items: Vec<ExperienceBonusMapItem>,
    pub chunks: Vec<Chunk>,
    pub current_chapter_index: i64,
    pub goals_book_bookmarks: Vec<GoalsBookBookmark>,
    pub chapters_groups: Vec<ChaptersGroup>,
    pub growing_spots_main: Vec<GrowingSpotsMain>,
    pub growing_spots_tutorial: Vec<Value>,
    pub pointer_state: i64,
    pub pointer_direction: i64,
    pub pointer_position: f64,
    pub difficulty_to_select: i64,
    pub current_difficulty: i64,
    pub npc_bonuses: NpcBonuses,
    pub npc_haggle_themes: NpcHaggleThemes,
    pub npc_haggle_seeds: NpcHaggleSeeds,
    pub current_bonuses: Vec<CurrentBonuse>,
    pub state_before_haggle_started: i64,
    pub haggle_state: i64,
    pub unlocked_difficulties: Vec<i64>,
    pub items_from_inventory: Vec<ItemsFromInventory>,
    pub karma: i64,
    pub npc_queue: Vec<NpcQueue>,
    pub spawned_npc_queue: Vec<Value>,
    pub npc_global_settings: NpcGlobalSettings,
    pub npc_who_never_come: Vec<String>,
    pub current_npc: CurrentNpc,
    pub tutorial_npc_queue_count: i64,
    pub was_first_main_trader_spawned: bool,
    pub karmic_traders_virtual_queue: KarmicTradersVirtualQueue,
    pub extra_traders_virtual_queue: ExtraTradersVirtualQueue,
    pub main_traders_spawn_repeat_defender: MainTradersSpawnRepeatDefender,
    pub quests_on_cooldown: QuestsOnCooldown,
    pub pestle_position: PestlePosition,
    pub pestle_rotation: PestleRotation,
    pub gold: i64,
    pub gold_earned: i64,
    pub profit_from_haggling: i64,
    pub player_panel_type: i64,
    pub player_sort_type: i64,
    pub player_element_type: i64,
    pub player_reversed_sort: bool,
    pub player_inventory: Vec<PlayerInventory>,
    pub player_trading_panel_inventory: Vec<Value>,
    pub popularity: i64,
    pub potion_effect_map_items: Vec<PotionEffectMapItem>,
    pub saved_recipes: Vec<SavedRecipe>,
    pub recipe_book_pages_to_add: i64,
    pub current_recipe_index: i64,
    pub current_potion_recipe_index: i64,
    pub unlocked_recipes_pages_count: i64,
    pub recipe_book_bookmarks: Vec<RecipeBookBookmark>,
    pub zoom: f64,
    pub map_position: MapPosition,
    pub follow_indicator_button_state: bool,
    pub selected_potion_base_name: String,
    pub default_potion_base_name: String,
    pub unlocked_potion_bases: Vec<String>,
    pub unlocked_potion_bases_read: Vec<bool>,
    pub vortexes: Vec<Vortex>,
    pub current_room: i64,
    pub is_player_visited_meeting_room_after_day_start: bool,
    pub scales: Scales,
    pub scales_potion_item: ScalesPotionItem,
    pub is_current_potion_suitable: bool,
    pub is_potion_item_on_scales: bool,
    pub reset_scales: bool,
    pub spoon_position: SpoonPosition,
    pub spoon_rotation: SpoonRotation,
    pub spoon_animation_frame_number: i64,
    pub spoon_dirty_parts_list: Vec<SpoonDirtyPartsList>,
    pub seconds_played: f64,
    pub clients_served: i64,
    pub potions_brewed: i64,
    pub talents: Talents,
    pub show_talents_notification: bool,
    pub trader_inventory: Vec<Value>,
    pub trader_panel_type: i64,
    pub trader_sort_type: i64,
    pub trader_element_type: i64,
    pub trader_reversed_sort: bool,
    pub trader_trading_panel_inventory: Vec<Value>,
    pub tutorial_state: TutorialState,
    #[serde(rename = "FahlgorithmAlchemyMachineRecipes")]
    pub fahlgorithm_alchemy_machine_recipes: Vec<FahlgorithmAlchemyMachineRecipe>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemsDaum {
    pub system_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slots {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<String>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<MValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MValue {
    pub name: String,
    pub count: i64,
    pub class_full_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendaryRecipeBookmark {
    pub serialized_rails: Vec<SerializedRail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRail {
    pub serialized_bookmarks: Vec<SerializedBookmark>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedBookmark {
    pub position: Position,
    pub prefab_index: i64,
    pub is_mirrored: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoalsHeat {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<String>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedColor {
    pub color: Color,
    pub hsv: Hsv,
    pub is_default_color: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hsv {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorsHistory {
    pub color: Color,
    pub hsv: Hsv,
    pub is_default_color: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedCurrentPotion {
    pub potion_used_components: Vec<Value>,
    pub recipe_marks: Vec<RecipeMark>,
    pub collected_potion_effects: Vec<Value>,
    pub potion_skin_settings: PotionSkinSettings,
    pub serialized_path: SerializedPath,
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
    pub fixed_path_points: Vec<Value>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedDefaultBottleSettings {
    pub default_bottle_weak_potion_name: String,
    pub default_sticker_weak_potion_name: String,
    pub default_sticker_angle_weak_potion: i64,
    pub default_bottle_normal_potion_name: String,
    pub default_sticker_normal_potion_name: String,
    pub default_sticker_angle_normal_potion: i64,
    pub default_bottle_strong_potion_name: String,
    pub default_sticker_strong_potion_name: String,
    pub default_sticker_angle_strong_potion: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousPotionRecipe {
    pub name: String,
    pub class_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
    pub current_exp: f64,
    pub next_lvl_at: f64,
    pub current_lvl_at: f64,
    pub current_lvl: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceBonusMapItem {
    pub local_position: LocalPosition,
    pub already_collected: bool,
    pub is_daily: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    pub map_index: i64,
    pub position_in_grid: PositionInGrid,
    pub sum_of_tiles_fog_density: i64,
    pub tiles: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInGrid {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalsBookBookmark {
    pub serialized_rails: Vec<SerializedRail2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRail2 {
    pub serialized_bookmarks: Vec<SerializedBookmark2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedBookmark2 {
    pub position: Position2,
    pub prefab_index: i64,
    pub is_mirrored: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChaptersGroup {
    pub name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub name: String,
    pub chapter_goal: ChapterGoal,
    pub goals: Vec<Goal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterGoal {
    pub name: String,
    pub progress: i64,
    pub follow_state: i64,
    pub is_completion_read: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub name: String,
    pub progress: i64,
    pub follow_state: i64,
    pub is_completion_read: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowingSpotsMain {
    pub spot_name: String,
    pub plant_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcBonuses {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<i64>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<MValue2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MValue2 {
    pub infos: Vec<Info>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub size: f64,
    pub theme_title: String,
    pub bonus_index: i64,
    pub size_index: i64,
    pub mirrored: bool,
    pub position: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcHaggleThemes {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<i64>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcHaggleSeeds {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<i64>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentBonuse {
    pub size: f64,
    pub theme_title: String,
    pub bonus_index: i64,
    pub size_index: i64,
    pub mirrored: bool,
    pub position: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsFromInventory {
    pub type_name: String,
    pub position: Position3,
    pub euler_angles: EulerAngles,
    pub inventory_item_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position3 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EulerAngles {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcQueue {
    pub npc_template_name: String,
    pub npc_faction_name: String,
    pub npc_faction_class_name: String,
    pub npc_mood: i64,
    pub chapter_on_add_to_spawn: i64,
    pub path_tween_full_position: f64,
    pub pause_before_spawn: bool,
    pub random_states_container: RandomStatesContainer,
    pub spawn_position: SpawnPosition,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomStatesContainer {
    pub states_keys: Vec<String>,
    pub states_values: Vec<StatesValue>,
    pub resettable_states: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatesValue {
    pub s0: i64,
    pub s1: i64,
    pub s2: i64,
    pub s3: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcGlobalSettings {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<String>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<MValue3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MValue3 {
    pub closeness: i64,
    pub cooldown: i64,
    pub npc_has_visited_shop_at_closeness: Vec<i64>,
    pub completed_unique_quests_at_closeness: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentNpc {
    pub npc: Npc,
    pub potion: Potion,
    pub is_potion_sold: bool,
    pub is_haggle_canceled: bool,
    pub is_current_deal_haggled: bool,
    pub is_any_deal_haggled: bool,
    pub gold: i64,
    pub closeness: i64,
    pub bargained: i64,
    pub random_greetings_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub npc_for_spawn: NpcForSpawn,
    pub state: i64,
    pub quests_on_cooldown_on_spawn: QuestsOnCooldownOnSpawn,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcForSpawn {
    pub npc_template_name: String,
    pub npc_faction_name: String,
    pub npc_faction_class_name: String,
    pub npc_mood: i64,
    pub chapter_on_add_to_spawn: i64,
    pub path_tween_full_position: f64,
    pub pause_before_spawn: bool,
    pub random_states_container: RandomStatesContainer2,
    pub spawn_position: SpawnPosition2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomStatesContainer2 {
    pub states_keys: Vec<String>,
    pub states_values: Vec<StatesValue2>,
    pub resettable_states: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatesValue2 {
    pub s0: i64,
    pub s1: i64,
    pub s2: i64,
    pub s3: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnPosition2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestsOnCooldownOnSpawn {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<String>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Potion {
    pub type_name: String,
    pub position: Position4,
    pub euler_angles: EulerAngles2,
    pub inventory_item_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position4 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EulerAngles2 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KarmicTradersVirtualQueue {
    pub queue: Queue,
    pub temporary_pool: Vec<Value>,
    pub first_day_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<Value>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraTradersVirtualQueue {
    pub queue: Queue2,
    pub temporary_pool: Vec<Value>,
    pub first_day_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue2 {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<i64>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainTradersSpawnRepeatDefender {
    pub spawned_npc: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestsOnCooldown {
    #[serde(rename = "m_keys")]
    pub m_keys: Vec<String>,
    #[serde(rename = "m_values")]
    pub m_values: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PestlePosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PestleRotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventory {
    pub name: String,
    pub count: i64,
    pub class_full_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotionEffectMapItem {
    pub local_position: LocalPosition2,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPosition2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedRecipe {
    pub name: String,
    pub class_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeBookBookmark {
    pub serialized_rails: Vec<SerializedRail3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRail3 {
    pub serialized_bookmarks: Vec<SerializedBookmark3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedBookmark3 {
    pub position: Position5,
    pub prefab_index: i64,
    pub is_mirrored: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position5 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vortex {
    pub is_locked: bool,
    pub position: Position6,
    pub is_indicator_moving_through_vortex: bool,
    pub path_shift: PathShift,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position6 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathShift {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scales {
    pub fluctuation_time: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub amplitudes_sum: f64,
    pub is_fluctuating: bool,
    pub is_shaking: bool,
    pub is_shaking_to_the_right: bool,
    pub moving_side_sign: i64,
    pub is_correct_bonus_clicked: bool,
    pub is_incorrect_bonus_clicked: bool,
    pub time_coefficient: f64,
    pub is_lowest_position_on_haggle_reached: bool,
    pub is_wrong_potion_on_the_scales: bool,
    pub cap_angle_on_return_from_max_angle: bool,
    pub current_angle: f64,
    pub target_angle: f64,
    pub enable_physics: bool,
    pub current_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalesPotionItem {
    pub name: String,
    pub count: i64,
    pub class_full_name: String,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpoonPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpoonRotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpoonDirtyPartsList {
    pub mask_local_position: MaskLocalPosition,
    pub part_color: PartColor,
    pub contamination_value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskLocalPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Talents {
    pub current_points: i64,
    pub earned_talents_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TutorialState {
    pub set_name: String,
    pub step_index: i64,
    pub manipulated_objects_lock: Vec<Value>,
    pub reset_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FahlgorithmAlchemyMachineRecipe {
    #[serde(rename = "Index")]
    pub index: i64,
    #[serde(rename = "Recipe")]
    pub recipe: Recipe,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub name: String,
    pub class_name: String,
    pub data: String,
}
