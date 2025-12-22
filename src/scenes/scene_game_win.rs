use crate::puzzle_size::PuzzleSize;
use crate::sfx::stop_bgm;
use crate::{bg_gfx, sprites, Scene, SceneAction, SceneMusic, SFX_CONGRATS};
use agb::display::object::{AffineMatrixObject, AffineMode, GraphicsMode, Object, ObjectAffine, Sprite};
use agb::display::tiled::RegularBackgroundSize::Background32x32;
use agb::display::tiled::{RegularBackground, TileFormat, VRAM_MANAGER};
use agb::display::{AffineMatrix, GraphicsFrame, Priority};
use agb::fixnum::{num, vec2, Num, Vector2D};
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{ChannelId, Mixer, SoundChannel};
use alloc::boxed::Box;
use core::ops::Sub;

const DURATION: i32 = 50;
const IMG_DURATION: f32 = 80.0;

//                                 C   o   n   g   r   a   t   u   l  a   t   i  o   n   s   !
const LETTER_SPACING: [i32; 16] = [14, 12, 12, 14, 10, 14, 10, 14, 7, 12, 10, 8, 12, 14, 12, 0];

pub struct GameWinScene {
    anim_timer: u16,
    background: RegularBackground,
    puzzle_size: PuzzleSize,
    music_enabled: bool,
    puzzle_sprite: &'static Sprite,
    scale: Num<i32, 4>
}

impl GameWinScene {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(puzzle_size: PuzzleSize, game_idx: usize, music_enabled: bool) -> Box<dyn Scene> {
        let mut background =
            RegularBackground::new(Priority::P3, Background32x32, TileFormat::FourBpp);
        background.fill_with(&bg_gfx::win);

        Box::new(Self {
            anim_timer: 0,
            background,
            puzzle_size,
            music_enabled,
            puzzle_sprite: puzzle_size.images().sprite(game_idx),
            scale: num!(0.5)
        })
    }
}

impl Scene for GameWinScene {
    fn init(
        &mut self,
        bgm: Option<(SceneMusic, ChannelId)>,
        mixer: &mut Mixer,
    ) -> Option<(SceneMusic, ChannelId)> {
        VRAM_MANAGER.set_background_palettes(bg_gfx::PALETTES);
        if let Some(bgm) = bgm {
            stop_bgm(mixer, bgm);
        }
        None
    }

    fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) -> Option<SceneAction> {
        self.scale = self.scale.sub(num!(0.01)).max(num!(0.5));
        if self.music_enabled && self.anim_timer == 4 {
            let mut music = SoundChannel::new_high_priority(SFX_CONGRATS);
            music.stereo();
            mixer.play_sound(music);
        }
        if self.anim_timer > DURATION as u16
            && (buttons.is_just_pressed(Button::A) || buttons.is_just_pressed(Button::B))
        {
            return Some(SceneAction::PuzzleMenu(self.puzzle_size));
        }
        None
    }

    fn show(&mut self, graphics: &mut GraphicsFrame) {
        let obj = ObjectAffine::new(
            self.puzzle_sprite,
            AffineMatrixObject::new(AffineMatrix::from_scale(Vector2D::new(self.scale, self.scale))),
            AffineMode::AffineDouble,
        );

        draw_bg_and_image(
            &mut self.anim_timer,
            self.puzzle_size,
            obj,
            &self.background,
            graphics,
        );

        draw_congrats(self.anim_timer, graphics);
    }
}

fn draw_bg_and_image(
    anim_timer: &mut u16,
    puzzle_size: PuzzleSize,
    mut puzzle_sprite: ObjectAffine,
    background: &RegularBackground,
    graphics: &mut GraphicsFrame,
) {
    let id = background.show(graphics);
    *anim_timer += 1;

    let perc = ((*anim_timer as f32) / IMG_DURATION).min(1.0);

    let (x, y) = match puzzle_size {
        PuzzleSize::_12x12 | PuzzleSize::_10x10 | PuzzleSize::_6x6 | PuzzleSize::_8x8 => (104, 36),
        PuzzleSize::_20x10 | PuzzleSize::_22x12 => (88, 32),
    };

    puzzle_sprite
        .set_pos(vec2(x, y))
        .set_graphics_mode(GraphicsMode::AlphaBlending)
        .show(graphics);

    graphics
        .blend()
        .object_transparency(Num::from_f32(perc), Num::from_f32(1.0 - perc))
        .enable_background(id);
}

fn draw_congrats(anim_timer: u16, graphics: &mut GraphicsFrame) {
    let mut offset = 0;
    let y = 100;
    let start_x = 240;
    let end_x = 30;
    for i in 0..16 {
        let duration = DURATION + (i * 3);
        let x = lerp(start_x, end_x + offset, anim_timer as i32, duration);
        Object::new(sprites::CONGRATS.sprite(i as usize))
            .set_pos((x, y))
            .show(graphics);
        offset += LETTER_SPACING[i as usize];
    }
}

#[inline]
const fn lerp(start: i32, end: i32, val: i32, duration: i32) -> i32 {
    if val > duration {
        return end;
    }
    start + (end - start) * val / duration
}
