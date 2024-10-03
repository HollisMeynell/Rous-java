package rosu

import rosu.osu.Mode
import rosu.parameter.JniScoreState
import rosu.result.*
import java.nio.ByteBuffer

internal object JniProcessor {
    private const val ERROR: UByte = 0b10000000u
    private const val Osu: UByte = 0b00000001u
    private const val Taiko: UByte = 0b00000010u
    private const val Catch: UByte = 0b00000100u
    private const val Mania: UByte = 0b00001000u

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
                throw Exception(buffer.readString())
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
                throw Exception(buffer.readString())
            }
            else -> throw Exception("Unknown mode")
        }

        return result
    }

    private fun ByteBuffer.readString(): String {
        val length = int
        val bytes = ByteArray(length)
        get(bytes)
        return String(bytes)
    }

    fun readJniBytes(bytes: ByteArray): ByteArray = bytes.apply {
        if (isEmpty()) throw Exception("Empty bytes")
        val data = slice(1..<size).toByteArray()
        if (get(0).toUByte() == ERROR) {
            throw Exception(ByteBuffer.wrap(data).readString())
        }
        return data
    }
}