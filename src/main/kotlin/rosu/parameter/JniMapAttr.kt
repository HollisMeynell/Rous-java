package rosu.parameter

import rosu.osu.Mode

data class JniMapAttr(
    var mode: Mode = Mode.Default,
    var mods: Int = 0,
    var speed: Double = -1.0,
    var accuracy: Double = 0.0,
    var isLazer: Boolean = false,

    var ar: Double = -21.0,
    var od: Double = -21.0,
    var cs: Double = -21.0,
    var hp: Double = -21.0,
) : Parameter {
    override fun size(): Int = 1 + 4 + 8 + 8 + 1 + (4 * 8)

    override fun toBytes() = buffer {
        put(mode.getValue().toByte())
        putInt(mods)
        putDouble(speed)
        putDouble(accuracy)
        if (isLazer) {
            put(1)
        } else {
            put(0)
        }

        putDouble(ar)
        putDouble(od)
        putDouble(cs)
        putDouble(hp)
    }
}
