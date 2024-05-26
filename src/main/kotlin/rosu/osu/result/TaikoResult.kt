package rosu.osu.result

import rosu.osu.JniResult
import rosu.osu.Mode

class TaikoResult : JniResult {
    override var mode: Mode = Mode.Taiko
    override var pp: Double = 0.0
    override var star: Double = 0.0
    override var combo: Int = 0

    var ppAcc: Double = 0.0
    var ppDifficulty: Double = 0.0
}