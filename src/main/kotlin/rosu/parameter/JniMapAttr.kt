package rosu.parameter

import rosu.osu.Mode
import java.nio.ByteBuffer

data class JniMapAttr(
    var mode: Mode = Mode.Default,
    var mods: Int = 0,
    var speed: Double = -1.0,
    var accuracy: Double = 0.0,
) : Parameter {
    override fun size(): Int  = 1 + 4 + 8 + 8

    override fun toBytes(): ByteArray {
        val buffer = ByteBuffer.allocate(size())
        if (mode != Mode.Default) {
            buffer.put(mode.getValue().toByte())
        } else {
            buffer.put(4.toByte())
        }
        buffer.putInt(mods)
        buffer.putDouble(speed)
        buffer.putDouble(accuracy)
        return buffer.array()
    }
}
