use std::io::Read;
use std::time;
use rosu_pp::{Beatmap, Difficulty, Performance};
use rosu_pp::any::ScoreState;
use rosu_pp::model::mode::GameMode;
bitflags::bitflags! {
    struct StatusFlag :u8 {
        const Error = 0u8;
        const Osu = 0b00000001u8;
        const Taiko = 0b00000010u8;
        const Catch = 0b00000100u8;
        const Mania = 0b00001000u8;
    }
}

fn main() {

    let mut file = std::fs::File::open("F:/bot/osufile/2993974.osu").unwrap();
    let mut fdata = vec![];
    file.read_to_end(&mut fdata).unwrap();
    let start = time::Instant::now();
    do_work(&fdata);
    let t = start.elapsed();
    println!("run {:?}", t);
}

fn do_work(data: &[u8]) {
    let mut map = Beatmap::from_bytes(&data).unwrap();
    map.mode = GameMode::Mania;
    let difficulty = Difficulty::new().calculate(&map);
    let start = difficulty.stars();
    let combo = difficulty.max_combo();
    println!("max combo {}", combo);
    let performance = Performance::new(difficulty)
        .state(ScoreState{
            max_combo: 958,
            n_geki: 2696,
            n_katu: 0,
            n300: 12,
            n100: 0,
            n50: 0,
            misses: 11,
        })
        ;
    let pp = match performance {
        Performance::Osu(osu) => {
            osu.calculate().pp
        }
        Performance::Taiko(taiko) => {
            taiko.calculate().pp
        }
        Performance::Catch(catch) => {
            catch.calculate().pp
        }
        Performance::Mania(mania) => {
            mania.calculate().pp
        }
    };

    println!("pp:{}, start:{}", pp, start)
}