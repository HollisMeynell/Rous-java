package rosu.osu

import rosu.osu.result.CatchResult
import rosu.osu.result.ManiaResult
import rosu.osu.result.OsuResult
import rosu.osu.result.TaikoResult
import java.nio.ByteBuffer

interface JniResult {
    var mode: Mode
    var pp: Double
    var star: Double
    var combo: Int

    companion object {
        const val ERROR: UByte = 0u
        const val Osu: UByte = 0b00000001u
        const val Taiko: UByte = 0b00000010u
        const val Catch: UByte = 0b00000100u
        const val Mania: UByte = 0b00001000u

        @JvmStatic
        fun fromBytes(bytes: ByteArray): JniResult {

            val buffer = ByteBuffer.wrap(bytes)
            val head = buffer.get().toUByte()
            val result:JniResult = when (head) {
                Osu -> {
                    val osuResult = OsuResult()
                    osuResult.pp = buffer.double
                    osuResult.star = buffer.double
                    osuResult.combo = buffer.int

                    osuResult.ppAcc = buffer.double
                    osuResult.ppAim = buffer.double
                    osuResult.ppSpeed = buffer.double
                    osuResult.ppFlashlight = buffer.double
                    osuResult
                }
                Taiko -> {
                    val taikoResult = TaikoResult()
                    taikoResult.pp = buffer.double
                    taikoResult.star = buffer.double
                    taikoResult.combo = buffer.int

                    taikoResult.ppAcc = buffer.getDouble()
                    taikoResult.ppDifficulty = buffer.getDouble()
                    taikoResult
                }
                Catch -> {
                    val catchResult = CatchResult()
                    catchResult.pp = buffer.double
                    catchResult.star = buffer.double
                    catchResult.combo = buffer.int
                    catchResult
                }
                Mania -> {
                    val maniaResult = ManiaResult()
                    maniaResult.pp = buffer.double
                    maniaResult.star = buffer.double
                    maniaResult.combo = buffer.int

                    maniaResult.ppDifficulty = buffer.double
                    maniaResult
                }
                ERROR -> {
                    throw Exception(readString(buffer))
                }
                else -> throw Exception("Unknown mode")
            }

            return result
        }

        @JvmStatic
        fun readString(buffer: ByteBuffer): String {
            val length = buffer.get().toInt()
            val bytes = ByteArray(length)
            buffer.get(bytes)
            return String(bytes)
        }
    }
}