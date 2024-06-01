package rosu.result

import rosu.osu.Mode

interface JniResult {
    var mode: Mode
    var pp: Double
    var star: Double
    var combo: Int
}