package rosu.result

import rosu.osu.Mode
import rosu.parameter.JniMapAttr
import rosu.parameter.JniScore
import rosu.parameter.JniScoreState

class JniCalculate (
    val ptr: Long,
    val mode: Mode,
    val mods: Int,
    val score: JniScoreState
) {
    fun getJniScore(): JniScore {
        return JniScore(
            attr = JniMapAttr(
                mode = mode,
                mods = mods
            ),
            state = score
        )
    }
}