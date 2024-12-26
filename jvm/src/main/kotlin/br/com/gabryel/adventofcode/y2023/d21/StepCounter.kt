package br.com.gabryel.adventofcode.y2023.d21

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.y2023.d21.area.Area
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import br.com.gabryel.adventofcode.y2023.d21.area.SingleField

fun main() {
    listOf("sample", "input").forEach { file ->
        val map = readLines(2023, 21, file)

        SingleStepCounter(map).printSequenceForSteps(file, "Limited", 6, 64)
        BigStepCounter(map).printSequenceForSteps(file, "Infinite", 6, 10, 50, 26501365)
    }
}

private fun StepCounter.printSequenceForSteps(file: String, type: String, vararg printable: Long) {
    printable.forEach { steps ->
        logTimeSection("[Walkable Tiles - $type][$file][$steps]") {
            println("[Walkable Tiles - $type][$file][$steps] ${getPossibleTilesOn(steps)}")
        }
    }
}

interface StepCounter {
    fun getPossibleTilesOn(steps: Long): Long
}

class SingleStepCounter(private val map: List<String>) : StepCounter {
    override fun getPossibleTilesOn(steps: Long): Long {
        return map.getCentral().countPossibleAtStep(steps)
    }
}

class BigStepCounter(private val map: List<String>) : StepCounter {
    override fun getPossibleTilesOn(steps: Long): Long {
        return generateSequence<Area>(map.getCentral()) { it.grow() }
            .first { steps <= it.firstSignal }
            .countPossibleAtStep(steps)
    }
}

private fun List<String>.getCentral(): SingleField {
    val map = this.map { it.toCharArray() }.toTypedArray()
    val context = Context(map)

    val start = map.findFirst('S')
    return SingleField.from(context, listOf(0L to start))
}
