package rosu.parameter

import rosu.osu.Mode
import java.nio.ByteBuffer

@Suppress("unused")
data class JniScore(
    val attr: JniMapAttr = JniMapAttr(),
    val state: JniScoreState = JniScoreState(),
) : Parameter {
    constructor(
        mode: Mode = Mode.Default,
        mods: Int = 0,
        speed: Double = -1.0,
        accuracy: Double = 0.0,
        combo: Int = 0,
        geki: Int = 0,
        katu: Int = 0,
        n300: Int = 0,
        n100: Int = 0,
        n50: Int = 0,
        misses: Int = 0,
    ) : this(
        JniMapAttr(
            mode, mods, speed, accuracy
        ), JniScoreState(
            combo, geki, katu, n300, n100, n50, misses
        )
    )

    override fun size() = attr.size() + state.size()
    override fun toBytes(): ByteArray {
        val buffer = ByteBuffer.allocate(size())
        buffer.put(attr.toBytes())
        buffer.put(state.toBytes())
        return buffer.array()
    }
}