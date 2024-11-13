use crate::java::{Error, Result};
use crate::{to_ptr, to_status, to_status_use, StatusFlag};
use bytes::{Buf, BufMut, Bytes};
use jni::objects::JByteArray;
use jni::JNIEnv;
use rosu_pp::any::{DifficultyAttributes, PerformanceAttributes, ScoreState};
use rosu_pp::model::mode::GameMode;
use rosu_pp::{Beatmap, Difficulty, GameMods, GradualPerformance, Performance};
use std::ops::{Add, Not};
use rosu_pp::osu::OsuPerformance;

#[derive(Clone, Debug, PartialEq)]
pub struct JniMapAttributes {
    pub ar: f64,
    pub od: f64,
    pub cs: f64,
    pub hp: f64,
}

impl JniMapAttributes {
    fn to_data(self) -> Option<Self> {
        if self.ar < -20f64 && self.od < -20f64 && self.cs < -20f64 && self.hp < -20f64 {
            None
        } else {
            Some(self)
        }
    }

    #[inline]
    fn set_map_attr<T>(val: f64, s: T, action: impl FnOnce(T, f32, bool) -> T) -> T {
        if val >= -20.0 {
            action(s, val as f32, true)
        } else {
            s
        }
    }
}

impl From<&Beatmap> for JniMapAttributes {
    fn from(value: &Beatmap) -> Self {
        Self {
            ar: value.ar as f64,
            od: value.od as f64,
            cs: value.cs as f64,
            hp: value.hp as f64,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct JniAttributes {
    pub mode: Option<GameMode>,
    pub mods: u32,
    pub speed: f64,
    pub accuracy: f64,
    pub is_lazer: bool,
    pub map_attr: Option<JniMapAttributes>,
}

impl JniAttributes {
    pub fn mem_size() -> usize {
        22 + 32
    }

    pub fn difficulty(&self) -> Difficulty {
        let mut difficulty = Difficulty::new()
            .mods(self.mods);
        if let Some(map_attr) = &self.map_attr {
            difficulty = JniMapAttributes::set_map_attr(map_attr.ar, difficulty, Difficulty::ar);
            difficulty = JniMapAttributes::set_map_attr(map_attr.od, difficulty, Difficulty::od);
            difficulty = JniMapAttributes::set_map_attr(map_attr.cs, difficulty, Difficulty::cs);
            difficulty = JniMapAttributes::set_map_attr(map_attr.hp, difficulty, Difficulty::hp);
        }

        if self.speed > 0f64 {
            difficulty = difficulty.clock_rate(self.speed)
        }

        difficulty
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct JniScore {
    pub attr: JniAttributes,
    pub score: Option<ScoreState>,
}

impl JniScore {
    pub fn mem_size() -> usize {
        JniAttributes::mem_size() + 4 * 9
    }

    fn set_score_state<T>(value: u32, o: T, action: impl FnOnce(T, u32) -> T) -> T {
        if value > 0 {
            action(o, value)
        } else {
            o
        }
    }

    pub fn performance(self, map: Beatmap) -> Performance<'static> {
        let attributes = self.attr.difficulty().calculate(&map);

        let mut performance = Performance::new(attributes);

        performance = performance.mods(self.attr.mods);

        if self.attr.accuracy.is_zero().not() {
            performance = performance.accuracy(self.attr.accuracy);
        }

        if let Some(s) = self.score {
            let mut default_state = performance.generate_state();

            set_state(s.n_geki, &mut default_state, |a, b| { a.n_geki = b });
            set_state(s.n_katu, &mut default_state, |a, b| { a.n_katu = b });
            set_state(s.slider_tick_hits, &mut default_state, |a, b| { a.slider_tick_hits = b });
            set_state(s.slider_end_hits, &mut default_state, |a, b| { a.slider_end_hits = b });
            set_state(s.n300, &mut default_state, |a, b| { a.n300 = b });
            set_state(s.n100, &mut default_state, |a, b| { a.n100 = b });
            set_state(s.n50, &mut default_state, |a, b| { a.n50 = b });
            set_state(s.misses, &mut default_state, |a, b| { a.misses = b });
            set_state(s.max_combo, &mut default_state, |a, b| { a.max_combo = b });

            performance = performance.state(default_state)
        }

        performance
    }
}

impl Default for JniAttributes {
    fn default() -> Self {
        JniAttributes {
            mode: None,
            mods: 0,
            speed: 0.0,
            accuracy: 0.0,
            is_lazer: false,
            map_attr: None,
        }
    }
}

impl Default for JniScore {
    fn default() -> Self {
        JniScore {
            attr: JniAttributes::default(),
            score: None,
        }
    }
}

impl From<&[u8]> for JniAttributes {
    fn from(value: &[u8]) -> Self {
        if value.len() < JniAttributes::mem_size() {
            return JniAttributes::default();
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
        let is_lazer = bytes.get_u8() == 1;

        let ar = bytes.get_f64();
        let od = bytes.get_f64();
        let cs = bytes.get_f64();
        let hp = bytes.get_f64();

        let map_attr = JniMapAttributes { ar, od, cs, hp };

        JniAttributes {
            mode,
            mods,
            speed,
            accuracy,
            is_lazer,
            map_attr: map_attr.to_data(),
        }
    }
}

impl From<&[u8]> for JniScore {
    fn from(value: &[u8]) -> Self {
        let length = value.len();
        if length < JniAttributes::mem_size() {
            return JniScore::default();
        }

        let attr_size = JniAttributes::mem_size();
        let score_size = JniScore::mem_size();

        let attr = JniAttributes::from(&value[0..attr_size]);

        if length < score_size {
            return JniScore { attr, score: None };
        }

        let bytes = Bytes::copy_from_slice(&value[attr_size..score_size]);

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
/// - all: `[(mode)u8 | (pp)f64 | (star)f64 | (max combo)i32]`
/// - osu: `[(pp_{acc, aim, speed, fl})f64 * 4]`
/// - taiko: `[(pp_{acc, difficulty})f64 * 2]`
/// - mania: `[(pp_difficulty)f64]`
pub fn calculate(env: &JNIEnv, local_map: &JByteArray, score: &JByteArray) -> Result<Vec<u8>> {
    let (map, score) = get_map_and_score(env, local_map, score)?;
    let map_attr = JniMapAttributes::from(&map);
    let performance = score.performance(map);
    let mut result = Vec::<u8>::new();
    attr_to_bytes(&performance.calculate(), Some(&map_attr), &mut result);
    Ok(result)
}

/// 渐进 pp 的计算器
///
/// 获取 [`GradualPerformance`] 的指针
///
/// ` [(mode)u8 | (mods)i32 | (ptr)f64] `
pub fn get_calculate(env: &JNIEnv, local_map: &JByteArray, attr: &JByteArray) -> Result<Vec<u8>> {
    let (map, attr) = get_map_and_attr(env, local_map, attr)?;
    let mode = map.mode;
    let map_attr = JniMapAttributes::from(&map);
    let mods = attr.mods;

    let gradual = attr.difficulty().gradual_performance(&map);

    let ptr = to_ptr(gradual);
    let mut result = Vec::<u8>::new();
    calculate_to_bytes(ptr, mode, &map_attr, mods, &mut result);
    Ok(result)
}

/// 渐进计算 pp
///
/// ptr: [`GradualPerformance`] 的指针
///
/// 返回值与 [`calculate`] 相同
pub fn calculate_pp(env: &JNIEnv, ptr: i64, score: &JByteArray) -> Result<Vec<u8>> {
    let gradual = to_status_use::<GradualPerformance>(ptr)?;
    let mut score = get_score(env, score)?;
    if score.score.is_none() {
        return Err(Error::from("no score"));
    }
    let state = score.score.take().unwrap();
    let attr = gradual.next(state.clone());
    if attr.is_none() {
        return Err(Error::from("gradual error"));
    }
    let attr = attr.unwrap();

    let mut result = Vec::<u8>::new();
    attr_to_bytes(&attr, None, &mut result);

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
) -> Result<(Beatmap, JniAttributes)> {
    let mut map = get_map(env, local_map)?;
    let attr = get_map_attr(env, attr)?;

    if let Some(m) = attr.mode {
        map.convert_in_place(m);
    }

    Ok((map, attr))
}

fn get_map(env: &JNIEnv, local_map: &JByteArray) -> Result<Beatmap> {
    let map_bytes = env.convert_byte_array(local_map)?;
    let map = Beatmap::from_bytes(&map_bytes)?;
    Ok(map)
}

fn get_map_attr(env: &JNIEnv, attr: &JByteArray) -> Result<JniAttributes> {
    let attr_bytes = env.convert_byte_array(attr)?;
    let attr = JniAttributes::from(attr_bytes.as_slice());
    Ok(attr)
}

fn get_score(env: &JNIEnv, score: &JByteArray) -> Result<JniScore> {
    let score_bytes = env.convert_byte_array(score)?;
    let score = JniScore::from(score_bytes.as_slice());
    Ok(score)
}

fn bytes_to_score_state(mut bytes: Bytes) -> ScoreState {
    let slider_tick_hits = bytes.get_i32() as u32;
    let slider_end_hits = bytes.get_i32() as u32;
    let max_combo = bytes.get_i32() as u32;
    let n_geki = bytes.get_i32() as u32;
    let n_katu = bytes.get_i32() as u32;
    let n300 = bytes.get_i32() as u32;
    let n100 = bytes.get_i32() as u32;
    let n50 = bytes.get_i32() as u32;
    let misses = bytes.get_i32() as u32;

    ScoreState { max_combo, slider_tick_hits, slider_end_hits, n_geki, n_katu, n300, n100, n50, misses }
}

#[inline]
fn set_attr(
    attr: &PerformanceAttributes,
    map_attr: Option<&JniMapAttributes>,
    result: &mut dyn BufMut,
) {
    result.put_f64(attr.pp());
    result.put_f64(attr.stars());
    result.put_i32(attr.max_combo() as i32);

    if let Some(attr) = map_attr {
        result.put_i8(1);
        result.put_f64(attr.ar);
        result.put_f64(attr.od);
        result.put_f64(attr.cs);
        result.put_f64(attr.hp);
    } else {
        result.put_i8(0);
    }
}
fn attr_to_bytes(
    attr: &PerformanceAttributes,
    map_attr: Option<&JniMapAttributes>,
    result: &mut dyn BufMut,
) {
    match attr {
        PerformanceAttributes::Osu(data) => {
            result.put_u8(StatusFlag::Osu.bits());
            set_attr(attr, map_attr, result);

            result.put_f64(data.pp_acc);
            result.put_f64(data.pp_aim);
            result.put_f64(data.pp_speed);
            result.put_f64(data.pp_flashlight);
        }
        PerformanceAttributes::Taiko(data) => {
            result.put_u8(StatusFlag::Taiko.bits());
            set_attr(attr, map_attr, result);

            result.put_f64(data.pp_acc);
            result.put_f64(data.pp_difficulty);
        }
        PerformanceAttributes::Catch(_) => {
            result.put_u8(StatusFlag::Catch.bits());
            set_attr(attr, map_attr, result);
        }
        PerformanceAttributes::Mania(data) => {
            result.put_u8(StatusFlag::Mania.bits());
            set_attr(attr, map_attr, result);

            result.put_f64(data.pp_difficulty);
        }
    }
}

fn calculate_to_bytes(
    ptr: i64,
    mode: GameMode,
    map_attr: &JniMapAttributes,
    mods: u32,
    result: &mut dyn BufMut,
) {
    let head = match mode {
        GameMode::Osu => StatusFlag::Osu,
        GameMode::Taiko => StatusFlag::Taiko,
        GameMode::Catch => StatusFlag::Catch,
        GameMode::Mania => StatusFlag::Mania,
    };
    result.put_u8(head.bits());
    result.put_i32(mods as i32);
    result.put_f64(map_attr.ar);
    result.put_f64(map_attr.od);
    result.put_f64(map_attr.cs);
    result.put_f64(map_attr.hp);
    result.put_i64(ptr);
}

#[inline]
fn set_state(n: u32, state: &mut ScoreState, fx: fn(&mut ScoreState, u32)) {
    if n > 0 {
        fx(state, n);
    };
}

#[test]
fn b() -> Result<()> {
    let b = Beatmap::from_path("/home/spring/Documents/Agressor Bunx - Tornado (Original Mix) (Shadren) [Insane].osu")?;
    let mods = 1<<6 | 1<< 3;
    let diff = Difficulty::new()
        .mods(GameMods::from(mods));
    let a = diff.calculate(&b);

    let p = Performance::new(a)
        .mods(mods)
        .accuracy(94.89)
        .combo(252)
        .n300(299)
        .n100(22)
        .n50(1)
        .misses(1)
        .calculate();
    println!("{p:?}");
    Ok(())
}

fn a() -> Result<()> {
    let mut b = Beatmap::from_path("")?;

    let diff = Difficulty::new();
    let x = to_ptr(diff);
    println!("{x}");
    let diff = to_status::<Difficulty>(x)?;
    let n_diff = diff.ar(5f32, false)
        .hardrock_offsets(false)
        .clock_rate(50f64)
        .hp(3f32, false)
        .inspect().into_difficulty();
    let p = n_diff.calculate(&b).performance();

    let mods = GameMods::default();
    let attr = n_diff.calculate(&b);

    match attr {
        DifficultyAttributes::Osu(a) => {
            let p = a.stars;
        }
        DifficultyAttributes::Taiko(a) => {
            let p = a.stars;
        }
        DifficultyAttributes::Catch(a) => {}
        DifficultyAttributes::Mania(a) => {}
    }
    let mut p = Performance::new(&b);
    let state = p.generate_state();
    let c = p.calculate();
    match c {
        PerformanceAttributes::Osu(a) => {
            println!("{}", a.pp_aim);
        }
        PerformanceAttributes::Taiko(_) => {}
        PerformanceAttributes::Catch(_) => {}
        PerformanceAttributes::Mania(_) => {}
    }


    Ok(())
}

#[test]
fn test1() {
    let mut s = String::new();
    if let Ok(a) = std::io::stdin().read_line(&mut s) {
        s.trim().chars()
            .map(|c|{c as u8})
            .for_each(|c| println!("{c}"))
    }
}