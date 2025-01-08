package br.com.gabryel.adventofcode.y2023.d21

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.y2023.d21.area.Area
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import br.com.gabryel.adventofcode.y2023.d21.area.SingleField

fun countStepsFor(lines: List<String>, steps: Int) =
    SingleStepCounter(lines).getPossibleTilesOn(steps)

fun countStepsForInfinite(lines: List<String>, steps: Int) =
    BigStepCounter(lines).getPossibleTilesOn(steps)


interface StepCounter {
    fun getPossibleTilesOn(steps: Int): Long
}

class SingleStepCounter(private val map: List<String>) : StepCounter {
    override fun getPossibleTilesOn(steps: Int): Long {
        return map.getCentral().countPossibleAtStep(steps)
    }
}

class BigStepCounter(private val map: List<String>) : StepCounter {
    override fun getPossibleTilesOn(steps: Int): Long {
        return generateSequence<Area>(map.getCentral()) { it.grow() }
            .first { steps <= it.firstSignal }
            .countPossibleAtStep(steps)
    }
}

private fun List<String>.getCentral(): SingleField {
    val map = this.map { it.toCharArray() }.toTypedArray()
    val context = Context(map)

    val start = map.findFirst('S')
    return SingleField.from(context, listOf(0 to start))
}
