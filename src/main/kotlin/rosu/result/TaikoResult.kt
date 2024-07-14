package rosu.result

import rosu.osu.Mode

class TaikoResult : JniResult {
    override var mode: Mode = Mode.Taiko
    override var pp: Double = 0.0
    override var star: Double = 0.0
    override var combo: Int = 0

    var ppAcc: Double = 0.0
    var ppDifficulty: Double = 0.0

    override fun toString(): String {
        return "TaikoResult(mode=$mode, pp=$pp, star=$star, combo=$combo, ppAcc=$ppAcc, ppDifficulty=$ppDifficulty)"
    }
}