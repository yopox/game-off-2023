use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Game),
            )
            .add_collection_to_loading_state::<_, Textures>(GameState::Loading)
            .add_collection_to_loading_state::<_, Fonts>(GameState::Loading)
            .add_collection_to_loading_state::<_, Sounds>(GameState::Loading)
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 24., tile_size_y = 16., columns = 15, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "hero_S.png")]
    pub hero_s: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 32., columns = 15, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "hero_M.png")]
    pub hero_m: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 80., tile_size_y = 64., columns = 15, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "hero_L.png")]
    pub hero_l: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 13., tile_size_y = 28., columns = 2, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "old_guy.png")]
    pub old_guy: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "zombie_S.png")]
    pub zombie_s: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 17., tile_size_y = 20., columns = 2, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "zombie_2_L.png")]
    pub zombie_2_l: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 18., columns = 4, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "bird.png")]
    pub bird: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 80., columns = 7, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "boss_1.png")]
    pub boss_1: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 105., tile_size_y = 106., columns = 9, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "boss_2.png")]
    pub boss_2: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 132., tile_size_y = 131., columns = 37, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "final_boss.png")]
    pub boss_3: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 2, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "boss_1_eye.png")]
    pub boss_1_eye: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 9., tile_size_y = 8., columns = 2, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "boss_2_eye.png")]
    pub boss_2_eye: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 9., tile_size_y = 8., columns = 3, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "heart.png")]
    pub heart: Handle<TextureAtlas>,

    #[asset(path = "pixel.png")]
    pub pixel: Handle<Image>,

    #[asset(path = "cinema.png")]
    pub cinema: Handle<Image>,

    #[asset(path = "frame.png")]
    pub frame: Handle<Image>,

    #[asset(path = "new_heart.png")]
    pub new_heart: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
    #[asset(path = "fonts/NotJamChunky8.ttf")]
    pub chunky: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct Sounds {
    #[asset(path = "bgm/1_Intro.ogg")]
    pub intro: Handle<AudioSource>,

    #[asset(path = "bgm/2M_Caves.ogg")]
    pub caves_m: Handle<AudioSource>,
    #[asset(path = "bgm/2S_Caves.ogg")]
    pub caves_s: Handle<AudioSource>,
    #[asset(path = "bgm/2L_Caves.ogg")]
    pub caves_l: Handle<AudioSource>,

    #[asset(path = "bgm/2M_Caves Boss.ogg")]
    pub caves_boss_m: Handle<AudioSource>,
    #[asset(path = "bgm/2S_Caves Boss.ogg")]
    pub caves_boss_s: Handle<AudioSource>,

    #[asset(path = "bgm/3M_Forest.ogg")]
    pub forest_m: Handle<AudioSource>,
    #[asset(path = "bgm/3S_Forest.ogg")]
    pub forest_s: Handle<AudioSource>,
    #[asset(path = "bgm/3L_Forest.ogg")]
    pub forest_l: Handle<AudioSource>,

    #[asset(path = "bgm/3M_Forest Boss.ogg")]
    pub forest_boss_m: Handle<AudioSource>,
    #[asset(path = "bgm/3S_Forest Boss.ogg")]
    pub forest_boss_s: Handle<AudioSource>,
    #[asset(path = "bgm/3L_Forest Boss.ogg")]
    pub forest_boss_l: Handle<AudioSource>,

    #[asset(path = "bgm/4_Dramatic Tension.ogg")]
    pub tension: Handle<AudioSource>,

    #[asset(path = "bgm/4M_Final Boss.ogg")]
    pub final_boss_m: Handle<AudioSource>,
    #[asset(path = "bgm/4S_Final Boss.ogg")]
    pub final_boss_s: Handle<AudioSource>,
    #[asset(path = "bgm/4L_Final Boss.ogg")]
    pub final_boss_l: Handle<AudioSource>,

    #[asset(path = "bgm/5_Final Scene.ogg")]
    pub outro: Handle<AudioSource>,

    #[asset(path = "sfx/FX_AmbientBoss.ogg")]
    pub ambient_boss: Handle<AudioSource>,
    #[asset(path = "sfx/FX_BJump.ogg")]
    pub jump_l: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Boss Explosion.ogg")]
    pub boss_explosion: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Boss out.ogg")]
    pub boss_out: Handle<AudioSource>,
    #[asset(path = "sfx/FX_BPlayer hurt.ogg")]
    pub player_hurt_l: Handle<AudioSource>,
    #[asset(path = "sfx/FX_CollectSword.ogg")]
    pub collect_sword: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Dash.ogg")]
    pub dash: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Downsize.ogg")]
    pub downsize: Handle<AudioSource>,
    #[asset(path = "sfx/FX_FinalBossDie.ogg")]
    pub final_boss_die: Handle<AudioSource>,
    #[asset(path = "sfx/FX_FinalBossTransform.ogg")]
    pub final_boss_transform: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Game Over.ogg")]
    pub game_over: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Heal.ogg")]
    pub heal: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Hurt.ogg")]
    pub hurt: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Jump.ogg")]
    pub jump: Handle<AudioSource>,
    #[asset(path = "sfx/FX_MobHurt.ogg")]
    pub mob_hurt: Handle<AudioSource>,
    #[asset(path = "sfx/FX_NPlayer hurt.ogg")]
    pub player_hurt_m: Handle<AudioSource>,
    #[asset(path = "sfx/FX_ObtainHeart.ogg")]
    pub obtain_heart: Handle<AudioSource>,
    #[asset(path = "sfx/FX_SPlayer hurt.ogg")]
    pub player_hurt_s: Handle<AudioSource>,
    #[asset(path = "sfx/FX_SJump.ogg")]
    pub jump_s: Handle<AudioSource>,

    #[asset(path = "sfx/FX_Step1.ogg")]
    pub step_1: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Step2.ogg")]
    pub step_2: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Step3.ogg")]
    pub step_3: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Step4.ogg")]
    pub step_4: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Step5.ogg")]
    pub step_5: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Step6.ogg")]
    pub step_6: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Step7.ogg")]
    pub step_7: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Step8.ogg")]
    pub step_8: Handle<AudioSource>,


    #[asset(path = "sfx/FX_Sword Finalboss.ogg")]
    pub sword_final_boss: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Sword ForestBoss.ogg")]
    pub sword_forest_boss: Handle<AudioSource>,
    #[asset(path = "sfx/FX_Sword.ogg")]
    pub sword: Handle<AudioSource>,

    #[asset(path = "sfx/FX_Upsize.ogg")]
    pub upsize: Handle<AudioSource>,
}