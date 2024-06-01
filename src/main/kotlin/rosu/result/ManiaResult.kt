package rosu.result

import rosu.osu.Mode

class ManiaResult : JniResult {
    override var mode: Mode = Mode.Mania
    override var pp: Double = 0.0
    override var star: Double = 0.0
    override var combo: Int = 0

    var ppDifficulty: Double = 0.0
}