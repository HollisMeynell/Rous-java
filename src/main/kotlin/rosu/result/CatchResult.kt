package rosu.result

import rosu.osu.Mode

class CatchResult : JniResult {
    override var mode: Mode = Mode.Catch
    override var pp: Double = 0.0
    override var star: Double = 0.0
    override var combo: Int = 0

    override fun toString(): String {
        return "CatchResult(mode=$mode, pp=$pp, star=$star, combo=$combo)"
    }
}