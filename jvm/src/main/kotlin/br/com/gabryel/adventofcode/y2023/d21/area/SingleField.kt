package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import br.com.gabryel.adventofcode.y2023.d21.area.CellType.*
import java.util.*
import kotlin.collections.ArrayDeque

private const val UNFILLED = -1L

class SingleField(
    override val context: Context,
    private val signals: Map<Direction, List<StepState>>,
    private val stepsMap: LongArray
) : Area {

    override val level = 1

    override val stepsToEnd = stepsMap.size.toLong()

    override val firstSignal = signals.values.minOf { it.minOf { it.first } }

    override fun getSignals(direction: Direction) = signals[direction]!!

    override fun getTimeToSignal(direction: Direction) = getSignals(direction).minOf { it.first }

    override fun expand(direction: Direction) = context.get(this, direction) {
        from(context, getSignals(direction))
    }

    override fun countPossibleAtStep(steps: Long) =
        if (steps < 0) 0
        else if (steps < stepsToEnd) stepsMap[steps.toInt()]
        else getPossiblePerParity(steps % 2 == 0L)

    private fun getPossiblePerParity(even: Boolean) =
        stepsMap[stepsMap.size - if ((stepsToEnd % 2 == 0L) == even) 2 else 1]

    companion object {
        fun from(context: Context, starts: List<StepState>): SingleField {
            val minTime = starts.minOf { it.first }
            val distances = Array(context.map.size) { LongArray(context.map.size) { UNFILLED } }

            val toVisit = ArrayDeque<StepState>().apply {
                this += starts.map { (startTime, start) -> (startTime - minTime) to start }
            }

            val allSignals = EnumMap<Direction, MutableList<StepState>>(Direction::class.java)

            while (toVisit.isNotEmpty()) {
                val (distance, tile) = toVisit.removeFirst()

                if (tile !in context.dimensions || distances[tile] != UNFILLED)
                    continue

                distances[tile] = distance

                val newDistance = distance + 1
                for ((coord, char, dir) in context.map.findAdjacent(tile)) {
                    when (char.getType()) {
                        GROUND -> toVisit += newDistance to coord
                        OUTSIDE -> allSignals.computeIfAbsent(dir) { mutableListOf() } +=
                            newDistance to coord.bindTo(context.dimensions)
                        else -> {}
                    }
                }
            }

            val signals = allSignals.mapValues { (_, directionSignals) -> directionSignals.filterRedundant() }
            return SingleField(context, signals, distances.createStepCounter())
        }

        private fun LongArray2D.createStepCounter(): LongArray {
            val stepsMap = getAll().filter { it != UNFILLED }.groupingBy { it }.eachCount()

            return (0..stepsMap.keys.max())
                .map { stepsMap[it] ?: 0 }.chunked(2)
                .scan(listOf(0L, 0L)) { (acc1, acc2), result ->
                    listOf(acc1 + result[0], acc2 + (result.getOrNull(1) ?: 0))
                }.drop(1).flatten().toList().toLongArray()
        }

        private fun Char?.getType(): CellType {
            return when (this) {
                '#' -> WALL
                null -> OUTSIDE
                else -> GROUND
            }
        }
    }

    override fun toString() = "[level 1, stepsToEnd: $stepsToEnd, hash: ${System.identityHashCode(this)}]"
}

private enum class CellType {
    WALL, GROUND, OUTSIDE
}
