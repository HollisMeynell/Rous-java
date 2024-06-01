package rosu

import rosu.parameter.JniMapAttr
import java.nio.file.Files
import kotlin.io.path.Path

fun main() {
    val f = Files.readAllBytes(Path("F:\\bot\\osufile\\4397861.osu"))
    val cal = Rosu.getCalculate(f, JniMapAttr())

    for (i in 0 until 1270) {
        cal.score.n300 += 1
        cal.score.combo += 2
        val result = Rosu.calculate(cal)
        println(result.pp)
    }
    Rosu.endCalculate(cal)
}