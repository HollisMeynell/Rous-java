package rosu.osu;

public enum class Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
    Default;
    companion object {
        fun getMode(i: Int) = when(i) {
            0 -> Osu
            1 -> Taiko
            2 -> Catch
            3 -> Mania
            else -> Default
        }
    }
    fun getValue() = when(this) {
        Osu -> 0
        Taiko -> 1
        Catch -> 2
        Mania -> 3
        Default -> -1
    }
}
