package rosu.osu

import java.nio.ByteBuffer

class JniScore (
    var mode: Mode = Mode.Default,
    var mods: Int = 0,
    var speed: Double = -1.0,
    var accuracy: Double = 0.0,
    var combo: Int = 0,
    var geki:Int = 0,
    var katu: Int = 0,
    var n300: Int = 0,
    var n100: Int = 0,
    var n50: Int = 0,
    var misses: Int = 0,
) {
    fun toBytes(): ByteArray {
        val buffer = ByteBuffer.allocate(1+8*2+4*8)
        if (mode != Mode.Default) {
            buffer.put(mode.getValue().toByte())
        } else {
            buffer.put(4.toByte())
        }
        buffer.putInt(mods)
        buffer.putDouble(speed)
        buffer.putDouble(accuracy)
        buffer.putInt(combo)
        buffer.putInt(geki)
        buffer.putInt(katu)
        buffer.putInt(n300)
        buffer.putInt(n100)
        buffer.putInt(n50)
        buffer.putInt(misses)
        return buffer.array()
    }
}