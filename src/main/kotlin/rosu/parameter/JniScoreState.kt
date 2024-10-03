package rosu.parameter

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

    override fun toBytes() = buffer {
        putInt(combo)
        putInt(geki)
        putInt(katu)
        putInt(n300)
        putInt(n100)
        putInt(n50)
        putInt(misses)
    }
}
