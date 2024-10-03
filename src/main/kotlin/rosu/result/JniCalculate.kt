package rosu.result

import rosu.Rosu
import rosu.osu.Mode
import rosu.parameter.JniMapAttr
import rosu.parameter.JniScore
import rosu.parameter.JniScoreState

class JniCalculate (
    pointer: Long,
    val mode: Mode,
    val mods: Int,
    val score: JniScoreState
) : AutoCloseable{
    private var ptr:Long? = pointer

    fun getJniScore(): JniScore {
        return JniScore(
            attr = JniMapAttr(
                mode = mode,
                mods = mods
            ),
            state = score
        )
    }

    override fun close() {
        if (ptr == null) return
        Rosu.releaseCalculate(this.ptr!!)
    }

    fun calculate(): JniResult {
        if (ptr == null) throw Error("Calculate is released")
        return Rosu.calculate(this.ptr!!, this.getJniScore().toBytes())
    }
}