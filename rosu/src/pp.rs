use bytes::{Buf, BufMut, Bytes};
use jni::JNIEnv;
use jni::objects::JByteArray;
use rosu_pp::{Beatmap, Difficulty, GradualPerformance, Performance};
use rosu_pp::any::{DifficultyAttributes, PerformanceAttributes, ScoreState};
use rosu_pp::model::mode::GameMode;

use crate::{StatusFlag, to_ptr, to_status_use};
use crate::java::{Error, Result};

pub struct JniMapAttr {
    pub mode: Option<GameMode>,
    pub mods: u32,
    pub speed: f64,
    pub accuracy: f64,
}

pub struct JniScore {
    pub attr: JniMapAttr,
    pub score: Option<ScoreState>,
}


impl JniScore {
    pub fn performance<'a>(self, attr: DifficultyAttributes) -> Performance<'a> {
        let max_combo = attr.max_combo();
        let mut p = Performance::new(attr);
        p = p.mods(self.attr.mods);

        if !self.attr.accuracy.is_zero() {
            p = p.accuracy(self.attr.accuracy);
        }

        if let Some(mut s) = self.score {
            if s.max_combo == 0 {
                s.max_combo = max_combo;
            }
            if s.n300 > 0 ||
                s.n100 > 0 ||
                s.n50 > 0 ||
                s.n_geki > 0 ||
                s.n_katu > 0 ||
                s.misses > 0 {
                p = p.state(s);
            }
            p
        } else {
            p.combo(max_combo)
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

        let mut bytes = Bytes::copy_from_slice(value);
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



/// 计算 pp, 如果没有成绩就是 map 的fc成绩
pub fn calculate(
    env: &JNIEnv,
    local_map: &JByteArray,
    score: &JByteArray,
) -> Result<Vec<u8>> {
    let (map, score) = get_map_and_score(env, local_map, score)?;

    let difficulty = Difficulty::new()
        .mods(score.attr.mods);
    let attributes = if score.attr.speed > 0.0 {
        difficulty.clock_rate(score.attr.speed).calculate(&map)
    } else {
        difficulty.calculate(&map)
    };

    let performance = score.performance(attributes);
    let mut result = Vec::<u8>::new();
    attr_to_bytes(&performance.calculate(), &mut result);
    Ok(result)
}

/// 渐进式计算成绩 获得计算器
pub fn get_calculate(
    env: &JNIEnv,
    local_map: &JByteArray,
    attr: &JByteArray,
) -> Result<Vec<u8>> {
    let (map, attr) = get_map_and_attr(env, local_map, attr)?;
    let mode = map.mode;
    let mods = attr.mods;
    let difficulty = Difficulty::new()
        .mods(mods);
    let gradual = if attr.speed > 0.0 {
        difficulty.clock_rate(attr.speed).gradual_performance(&map)
    } else {
        difficulty.gradual_performance(&map)
    };

    let ptr = to_ptr(gradual);
    let mut result = Vec::<u8>::new();
    calculate_to_bytes(ptr, mode, mods, &mut result);
    Ok(result)
}

/// 渐进计算 pp
/// `ptr` 计算器指针
pub fn calculate_pp(
    env: &JNIEnv,
    ptr: i64,
    score: &JByteArray,
) -> Result<Vec<u8>> {
    let gradual = to_status_use::<GradualPerformance>(ptr)?;
    let score = get_score(env, score)?;
    if score.score.is_none() {
        return Err(Error::from("no score"));
    }
    let state = score.score.as_ref().unwrap();
    let attr = gradual.next(state.clone());
    if attr.is_none() {
        return Err(Error::from("gradual error"));
    }
    let attr = attr.unwrap();

    let mut result = Vec::<u8>::new();
    attr_to_bytes(&attr, &mut result);

    Ok(result)
}

/// 从 java byte[] 读取 谱面/成绩 数据
fn get_map_and_score(
    env: &JNIEnv,
    local_map: &JByteArray,
    score: &JByteArray,
) -> Result<(Beatmap, JniScore)> {
    let mut map = get_map(env, local_map)?;
    let score = get_score(env, score)?;

    if let Some(m) = score.attr.mode {
        map.mode = m;
    }

    Ok((map, score))
}

/// 从 java byte[] 读取 谱面/Mods 数据
fn get_map_and_attr(
    env: &JNIEnv,
    local_map: &JByteArray,
    attr: &JByteArray,
) -> Result<(Beatmap, JniMapAttr)> {
    let mut map = get_map(env, local_map)?;
    let attr = get_map_attr(env, attr)?;

    if let Some(m) = attr.mode {
        map.mode = m;
    }

    Ok((map, attr))
}

fn get_map(env: &JNIEnv, local_map: &JByteArray) -> Result<Beatmap> {
    let map_bytes = env.convert_byte_array(local_map)?;
    let map = Beatmap::from_bytes(&map_bytes)?;
    Ok(map)
}

fn get_map_attr(env: &JNIEnv, attr: &JByteArray) -> Result<JniMapAttr> {
    let attr_bytes = env.convert_byte_array(attr)?;
    let attr = JniMapAttr::from(attr_bytes.as_slice());
    Ok(attr)
}

fn get_score(env: &JNIEnv, score: &JByteArray) -> Result<JniScore> {
    let score_bytes = env.convert_byte_array(score)?;
    let score = JniScore::from(score_bytes.as_slice());
    Ok(score)
}

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
