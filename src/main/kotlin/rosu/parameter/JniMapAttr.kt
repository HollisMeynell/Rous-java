package rosu.parameter

import rosu.osu.Mode

data class JniMapAttr(
    var mode: Mode = Mode.Default,
    var mods: Int = 0,
    var speed: Double = -1.0,
    var accuracy: Double = 0.0,
) : Parameter {
    override fun size(): Int  = 1 + 4 + 8 + 8

    override fun toBytes() = buffer {
        put(mode.getValue().toByte())
        putInt(mods)
        putDouble(speed)
        putDouble(accuracy)
    }
}
