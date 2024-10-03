package rosu.parameter

import rosu.osu.Mode

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
    override fun toBytes() = buffer {
        put(attr.toBytes())
        put(state.toBytes())
    }

    var mode: Mode by attr::mode

    var mods: Int by attr::mods

    var speed: Double by attr::speed

    var accuracy: Double by attr::accuracy

    var combo: Int by state::combo

    var geki: Int by state::geki

    var katu: Int by state::katu

    var n300: Int by state::n300

    var n100: Int by state::n100

    var n50: Int by state::n50

    var misses: Int by state::misses

}