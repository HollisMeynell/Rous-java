package rosu.osu

import java.io.DataInputStream

data class BeatmapAttributes(
    var version: Int?=null,
    var isConvert: Boolean?=null,
    var mode: Mode?=null,
    var ar: Float?=null,
    var cs: Float?=null,
    var hp: Float?=null,
    var od: Float?=null,
    var sliderMultiplier: Double?=null,
    var sliderTickRate: Double?=null,
) {
    companion object {
        @JvmStatic
        fun fromBytes(input: DataInputStream) : BeatmapAttributes {
            val result = BeatmapAttributes()
            result.version = input.readInt()
            result.isConvert = input.readBoolean()
            result.mode = Mode.getMode(input.readInt())

            result.ar = input.readFloat()
            result.cs = input.readFloat()
            result.hp = input.readFloat()
            result.od = input.readFloat()
            result.sliderMultiplier = input.readDouble()
            result.sliderTickRate = input.readDouble()
            return result
        }
    }
}
