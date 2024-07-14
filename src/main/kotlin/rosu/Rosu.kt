package rosu

import rosu.parameter.JniMapAttr
import rosu.parameter.JniScore
import rosu.result.JniCalculate
import rosu.result.JniResult

@Suppress("unused")
object Rosu {
    val version by lazy {
        val version = Rosu::class.java.getResourceAsStream("/git.version")
        version?.use {
            it.bufferedReader().readText().trim()
        } ?: ""
    }

    private val native = Native.instance
    @JvmStatic
    fun calculate(map: ByteArray, score: JniScore) : JniResult {
        val p = native.calculate(map, score.toBytes())
        return JniProcessor.bytesToResult(p)
    }

    @JvmStatic
    internal fun calculate(ptr: Long, scoreBytes: ByteArray) : JniResult {
        val p = native.calculateIterator(ptr, scoreBytes)
        return JniProcessor.bytesToResult(p)
    }
    @JvmStatic
    fun calculate(calculate: JniCalculate) : JniResult {
        return calculate.calculate()
    }

    @JvmStatic
    fun getCalculate(map: ByteArray, attr: JniMapAttr) : JniCalculate {
        val p = native.getCalculateIterator(map, attr.toBytes())
        return JniProcessor.bytesToCalculate(p)
    }

    @JvmStatic
    fun releaseCalculate(calculate: JniCalculate) {
        calculate.close()
    }

    internal fun releaseCalculate(ptr: Long) {
        val result = native.releaseCalculate(ptr)
        if (result.isNotEmpty()) throw Exception(String(result))
    }
}