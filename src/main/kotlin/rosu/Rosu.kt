package rosu

import rosu.parameter.JniMapAttr
import rosu.parameter.JniScore
import rosu.result.JniCalculate
import rosu.result.JniProcessor
import rosu.result.JniResult

@Suppress("unused")
object Rosu {
    private val native = Native()
    @JvmStatic
    fun calculate(map: ByteArray, score: JniScore) : JniResult {
        native.loadLib
        val p = native.calculate(map, score.toBytes())
        return JniProcessor.bytesToResult(p)
    }

    @JvmStatic
    fun calculate(calculate: JniCalculate) : JniResult {
        native.loadLib
        val p = native.calculateIterator(calculate.ptr, calculate.getJniScore().toBytes())
        return JniProcessor.bytesToResult(p)
    }

    @JvmStatic
    fun getCalculate(map: ByteArray, attr: JniMapAttr) : JniCalculate {
        native.loadLib
        val p = native.getCalculateIterator(map, attr.toBytes())
        return JniProcessor.bytesToCalculate(p)
    }

    @JvmStatic
    fun endCalculate(calculate: JniCalculate) {
        native.loadLib
        native.collectionCalculate(calculate.ptr)
    }

}