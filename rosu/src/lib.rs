use bytes::{Buf};
use rosu_pp::any::ScoreState;
use rosu_pp::model::mode::GameMode;
use rosu_pp::Performance;

pub mod java;
pub mod macros;

bitflags::bitflags! {
    struct StatusFlag :u8 {
        const Error = 0b10000000u8;
        const Osu = 0b00000001u8;
        const Taiko = 0b00000010u8;
        const Catch = 0b00000100u8;
        const Mania = 0b00001000u8;
    }
}

struct JniScore {
    mode: Option<GameMode>,
    mods: u32,
    speed:f64,
    accuracy: f64,
    score: ScoreState,
}

impl JniScore {
    pub fn performance(self, max_combo: u32, performance: Performance) -> Performance {
        let mut p = performance;
        p = p.mods(self.mods);

        if !self.accuracy.is_zero() {
            p = p.accuracy(self.accuracy);
        }

        let mut s = self.score.clone();
        if self.score.max_combo == 0 {
            s.max_combo = max_combo;
        }
        return p.state(s);
    }
}

impl From<&[u8]> for JniScore {
    fn from(value: &[u8]) -> Self {
        let mut bytes = bytes::Bytes::copy_from_slice(value);
        let mode = bytes.get_u8();
        let mode = if mode > 3 {
            None
        } else {
            Some(GameMode::from(mode))
        };
        let mods = bytes.get_i32() as u32;
        let speed = bytes.get_f64();
        let mut accuracy = bytes.get_f64();
        if accuracy.is_zero() {
            accuracy = 100f64;
        } else if accuracy < 1.001f64 {
            accuracy *= 100f64;
        }
        let max_combo = bytes.get_i32() as u32;
        let n_geki = bytes.get_i32() as u32;
        let n_katu = bytes.get_i32() as u32;
        let n300 = bytes.get_i32() as u32;
        let n100 = bytes.get_i32() as u32;
        let n50 = bytes.get_i32() as u32;
        let misses = bytes.get_i32() as u32;

        let score = ScoreState {
            max_combo,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            misses,
        };

        JniScore {
            mode,
            mods,
            speed,
            accuracy,
            score,
        }
    }
}

trait TestZero {
    fn is_zero(&self)->bool;
}

impl TestZero for f64 {
    fn is_zero(&self) -> bool {
        self.abs() < 1e-9
    }
}