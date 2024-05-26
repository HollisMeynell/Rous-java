package rosu.osu.result

import rosu.osu.JniResult
import rosu.osu.Mode

class CatchResult : JniResult {
    override var mode: Mode = Mode.Catch
    override var pp: Double = 0.0
    override var star: Double = 0.0
    override var combo: Int = 0
}