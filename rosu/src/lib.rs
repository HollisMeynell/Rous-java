use bytes::{Buf, BufMut, Bytes};
use rosu_pp::any::{PerformanceAttributes, ScoreState};
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

fn error_to_bytes(str: &str) -> Vec<u8> {
    let mut result = Vec::new();
    result.put_u8(StatusFlag::Error.bits());
    vec_add_str(str, &mut result);
    result
}

fn attr_to_bytes(attr: &PerformanceAttributes, result: &mut dyn BufMut) {
    match attr {
        PerformanceAttributes::Osu(data) => {
            result.put_u8(StatusFlag::Osu.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);

            result.put_f64(data.pp_acc);
            result.put_f64(data.pp_aim);
            result.put_f64(data.pp_speed);
            result.put_f64(data.pp_flashlight);
        }
        PerformanceAttributes::Taiko(data) => {
            result.put_u8(StatusFlag::Taiko.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);

            result.put_f64(data.pp_acc);
            result.put_f64(data.pp_difficulty);
        }
        PerformanceAttributes::Catch(data) => {
            result.put_u8(StatusFlag::Catch.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);
        }
        PerformanceAttributes::Mania(data) => {
            result.put_u8(StatusFlag::Mania.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);

            result.put_f64(data.pp_difficulty);
        }
    }
}

fn calculate_to_bytes(ptr: i64, mode: GameMode, mods: u32, result: &mut dyn BufMut) {
    let head = match mode {
        GameMode::Osu => {StatusFlag::Osu}
        GameMode::Taiko => {StatusFlag::Taiko}
        GameMode::Catch => {StatusFlag::Catch}
        GameMode::Mania => {StatusFlag::Mania}
    };
    result.put_u8(head.bits());
    result.put_i32(mods as i32);
    result.put_i64(ptr);
}

#[inline]
fn vec_add_str(str: &str, vec: &mut dyn BufMut) {
    let bytes = str.as_bytes();
    vec.put_i32(bytes.len() as i32);
    for b in bytes { vec.put_u8(*b); }
}

#[inline]
fn bytes_to_score_state(mut bytes: Bytes) -> ScoreState {
    let max_combo = bytes.get_i32() as u32;
    let n_geki = bytes.get_i32() as u32;
    let n_katu = bytes.get_i32() as u32;
    let n300 = bytes.get_i32() as u32;
    let n100 = bytes.get_i32() as u32;
    let n50 = bytes.get_i32() as u32;
    let misses = bytes.get_i32() as u32;

    ScoreState {
        max_combo,
        n_geki,
        n_katu,
        n300,
        n100,
        n50,
        misses,
    }
}

struct JniMapAttr {
    mode: Option<GameMode>,
    mods: u32,
    speed: f64,
    accuracy: f64,
}

struct JniScore {
    attr: JniMapAttr,
    score: Option<ScoreState>,
}

impl JniScore {
    pub fn performance(self, max_combo: u32, performance: Performance) -> Performance {
        let mut p = performance;
        p = p.mods(self.attr.mods);

        if !self.attr.accuracy.is_zero() {
            p = p.accuracy(self.attr.accuracy);
        }

        if let Some(mut s) = self.score {
            if s.max_combo == 0 {
                s.max_combo = max_combo;
            }
            p.state(s)
        } else {
            let mut s = ScoreState::default();
            s.max_combo = max_combo;
            p.state(s)
        }
    }
}

impl Default for JniMapAttr {
    fn default() -> Self {
        JniMapAttr {
            mode: None,
            mods: 0,
            speed: 0.0,
            accuracy: 0.0,
        }
    }
}

impl Default for JniScore {
    fn default() -> Self {
        JniScore {
            attr: JniMapAttr::default(),
            score: None,
        }
    }
}

impl From<&[u8]> for JniMapAttr {
    fn from(value: &[u8]) -> Self {
        if value.len() < 21 {
            return JniMapAttr::default();
        }

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

        JniMapAttr {
            mode,
            mods,
            speed,
            accuracy,
        }
    }
}

impl From<&[u8]> for JniScore {
    fn from(value: &[u8]) -> Self {
        let length = value.len();
        if length < 21 {
            return JniScore::default();
        }

        let attr = JniMapAttr::from(&value[0..21]);

        if length < 49 {
            return JniScore {
                attr,
                score: None,
            };
        }

        let bytes = Bytes::copy_from_slice(&value[21..49]);

        let score = bytes_to_score_state(bytes);

        JniScore {
            attr,
            score: Some(score),
        }
    }
}

trait TestZero {
    fn is_zero(&self) -> bool;
}

impl TestZero for f64 {
    fn is_zero(&self) -> bool {
        self.abs() < 1e-9
    }
}

#[test]
fn test_byte_to_jni_score() {
    let f = fs::read("F:\\bot\\attr").unwrap();
    let s = JniScore::from(f.as_slice());
    println!("s{}", s.attr.speed)
}

#[test]
fn box_use() {
    let mut t = Vec::new();
    t.push(1u8);
    t.push(6u8);
    t.push(3u8);
    t.push(12u8);

    let p = Box::new(t);
    let mut p = Box::into_raw(p);

    let mut t = java::to_status_use::<Vec<u8>>(p as i64);
    // let mut t = unsafe { &mut *p };
    t.push(0u8);
    t.push(1u8);
    println!("{:?}", t);

    let x = unsafe { Box::from_raw(p) };
    println!("{:?}", x.as_slice());
}