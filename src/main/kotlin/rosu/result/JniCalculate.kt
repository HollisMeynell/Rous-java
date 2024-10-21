package rosu.result

import rosu.Rosu
import rosu.osu.Mode
import rosu.parameter.JniMapAttr
import rosu.parameter.JniScore
import rosu.parameter.JniScoreState
import java.lang.ref.Cleaner

/**
 * @param ar from beatmap, mods do not affect
 * @param od from beatmap, mods do not affect
 * @param cs from beatmap, mods do not affect
 * @param hp from beatmap, mods do not affect
 */
class JniCalculate(
    pointer: Long,
    val mode: Mode,
    val mods: Int,

    val ar: Double = 0.0,
    val od: Double = 0.0,
    val cs: Double = 0.0,
    val hp: Double = 0.0,
    val score: JniScoreState
) : AutoCloseable {
    private var ptr: Long = pointer
    private var closed = false
    private val cleaner = CLEANER.register(this) {
        Rosu.releaseCalculate(ptr)
    }

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
        if (closed)  return
        closed = true
        cleaner.clean()
    }

    fun calculate(): JniResult {
        if (closed) throw Error("Calculate is released")
        return Rosu.calculate(this.ptr, this.getJniScore().toBytes())
    }

    private companion object {
        private val CLEANER = Cleaner.create()
    }
}