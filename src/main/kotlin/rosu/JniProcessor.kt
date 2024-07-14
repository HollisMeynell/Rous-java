package rosu

import rosu.osu.Mode
import rosu.parameter.JniScoreState
import rosu.result.*
import java.nio.ByteBuffer

internal object JniProcessor {
    const val ERROR: UByte = 0b10000000u
    const val Osu: UByte = 0b00000001u
    const val Taiko: UByte = 0b00000010u
    const val Catch: UByte = 0b00000100u
    const val Mania: UByte = 0b00001000u

    @JvmStatic
    fun bytesToCalculate(bytes: ByteArray): JniCalculate {
        val buffer = ByteBuffer.wrap(bytes)
        val head = buffer.get().toUByte()
        val mode = when (head) {
            Osu -> Mode.Osu
            Taiko -> Mode.Taiko
            Catch -> Mode.Catch
            Mania -> Mode.Mania
            ERROR -> {
                throw Exception(readString(buffer))
            }
            else -> throw Exception("Unknown mode")
        }
        val mods = buffer.int
        val ptr = buffer.long
        val result = JniCalculate(
            pointer = ptr,
            mode = mode,
            mods = mods,
            score = JniScoreState()
        )
        return result
    }

    @JvmStatic
    fun bytesToResult(bytes: ByteArray): JniResult {

        val buffer = ByteBuffer.wrap(bytes)
        val head = buffer.get().toUByte()
        val result: JniResult = when (head) {
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
        val length = buffer.getInt()
        val bytes = ByteArray(length)
        buffer.get(bytes)
        return String(bytes)
    }

    @JvmStatic
    fun readPointer(bytes: ByteArray): Long {
        val buffer = ByteBuffer.wrap(bytes)
        if (ERROR == buffer.get().toUByte()) {
            throw Exception(readString(buffer))
        }
        return buffer.getLong()
    }

    @JvmStatic
    fun readBytes(bytes: ByteArray): ByteArray {
        val buffer = ByteBuffer.wrap(bytes)
        if (ERROR == buffer.get().toUByte()) {
            throw Exception(readString(buffer))
        }
        return bytes.slice(1..<bytes.size).toByteArray()
    }
}