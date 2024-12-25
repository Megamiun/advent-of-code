package br.com.gabryel.adventofcode.y2023.d21

import br.com.gabryel.adventofcode.util.logTimeSection
import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.y2023.d21.area.Area
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import br.com.gabryel.adventofcode.y2023.d21.area.SingleField

fun main() {
    listOf("sample", "input").forEach { file ->
        val map = readLines(2023, 21, file)

        SingleStepCounter(map).printSequenceForSteps(file, "Limited", 6, 64)
        BigStepCounter(map).printSequenceForSteps(file, "Infinite", 6, 10, 50, 1000, 5000, 2650136, 26501365)
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
            .onEach { println("${it.level} -> ${it.stepsToEnd} -> ${it.stepsPerParity}") }
            .first { steps < it.firstOut }
            .countPossibleAtStep(steps)
    }
}

private fun List<String>.getCentral(): SingleField {
    val start = withIndex().flatMap { (y, row) ->
        row.withIndex().filter { (_, cell) -> cell == 'S' }.map { (x) -> x to y }
    }.first()

    return SingleField.from(Context(this.map { it.toCharArray() }.toTypedArray(), mutableMapOf()), listOf(0L to start))
}
