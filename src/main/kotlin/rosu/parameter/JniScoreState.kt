package rosu.parameter

data class JniScoreState(
    var combo: Int = 0,
    var geki: Int = 0,
    var katu: Int = 0,
    var n300: Int = 0,
    var n100: Int = 0,
    var n50: Int = 0,
    var misses: Int = 0,
) : Parameter {
    fun isEmpty(): Boolean {
        return combo == 0 &&
                geki == 0 &&
                katu == 0 &&
                n300 == 0 &&
                n100 == 0 &&
                n50 == 0 &&
                misses == 0
    }

    override fun size(): Int {
        return if (isEmpty()) {
            0
        } else {
            4 * 7
        }
    }

    override fun toBytes() = buffer {
        if (isEmpty()) return@buffer
        putInt(combo)
        putInt(geki)
        putInt(katu)
        putInt(n300)
        putInt(n100)
        putInt(n50)
        putInt(misses)
    }
}
