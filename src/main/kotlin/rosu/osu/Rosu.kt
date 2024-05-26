package rosu.osu

import rosu.Native

class Rosu {
    companion object {
        private val native = Native()
        @JvmStatic
        fun calculate(map: ByteArray, score: JniScore) : JniResult {
            native.loadLib
            val p = native.calculate(map, score.toBytes())
            return JniResult.fromBytes(p)
        }
    }
}