package rosu.parameter

import java.nio.ByteBuffer

data class JniScoreState(
    var combo: Int = 0,
    var geki:Int = 0,
    var katu: Int = 0,
    var n300: Int = 0,
    var n100: Int = 0,
    var n50: Int = 0,
    var misses: Int = 0,
) : Parameter {
    override fun size(): Int = 4*7

    override fun toBytes(): ByteArray {
        val buffer = ByteBuffer.allocate(size())
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
