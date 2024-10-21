package rosu.result

import rosu.osu.Mode

interface JniResult {
    var mode: Mode
    var pp: Double
    var star: Double
    var combo: Int

    var ar: Double
    var od: Double
    var cs: Double
    var hp: Double
}