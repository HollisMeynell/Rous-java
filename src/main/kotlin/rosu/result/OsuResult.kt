package rosu.result

import rosu.osu.Mode

class OsuResult : JniResult {
    override var mode: Mode = Mode.Osu
    override var pp: Double = 0.0
    override var star: Double = 0.0
    override var combo: Int = 0

    var ppAcc: Double = 0.0
    var ppAim: Double = 0.0
    var ppFlashlight: Double = 0.0
    var ppSpeed: Double = 0.0
}