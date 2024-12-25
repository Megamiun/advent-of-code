package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import br.com.gabryel.adventofcode.y2023.d21.area.CellType.*
import java.util.*
import kotlin.collections.ArrayDeque
import kotlin.collections.set

class SingleField(
    override val context: Context,
    private val signals: Map<Direction, List<StepState>>,
    private val stepsMap: LongArray
) : Area {

    override val level = 1

    override val stepsToEnd = stepsMap.size.toLong()

    override val firstOut = signals.values.minOf { it.minOf { it.first } }

    override fun getSignals(direction: Direction) = signals[direction]!!

    override fun getTimeToSignal(direction: Direction) = getSignals(direction).minOf { it.first }

    override fun expand(direction: Direction): Area = context.get(this, direction) {
        from(context, getSignals(direction))
    }

    override fun countPossibleAtStep(steps: Long) =
        if (steps < 0) 0
        else if (steps < stepsToEnd) stepsMap[steps.toInt()]
        else getPossiblePerParity(steps % 2 == 0L)

    override fun getPossiblePerParity(even: Boolean) =
        stepsMap[stepsMap.size - if (even) 2 else 1]

    companion object {
        fun from(context: Context, starts: List<StepState>): SingleField {
            val minTime = starts.minOf { it.first }
            val distances = mutableMapOf<Coordinate, Long>()

            val toVisit = ArrayDeque<StepState>().apply {
                this += starts.map { (startTime, start) -> (startTime - minTime) to start }
            }

            val signals = EnumMap<Direction, MutableList<StepState>>(Direction::class.java)
            val allSignals = EnumMap<Direction, MutableList<StepState>>(Direction::class.java)

            while (toVisit.isNotEmpty()) {
                val (distance, tile) = toVisit.removeFirst()

                if (distance >= (distances[tile] ?: Long.MAX_VALUE)) continue
                val newDistance = distance + 1

                for ((coord, typeDir) in context.map.findAdjacent(tile)) {
                    val (char, dir) = typeDir
                    when (char.getType()) {
                        OUTSIDE -> {
                            if (allSignals[dir].orEmpty().none { a -> Direction.entries.any { coord + it == a.second } }) {
                                signals.computeIfAbsent(dir) { mutableListOf() } += newDistance to (coord bindTo context.dimensions)
                            }
                            allSignals.computeIfAbsent(dir) { mutableListOf() } += newDistance to coord
                        }
                        GROUND -> {
                            distances[tile] = distance
                            toVisit += newDistance to coord
                        }
                        else -> {}
                    }
                }
            }

            return SingleField(context, signals, distances.createStepCounter())
        }

        private fun Map<Coordinate, Long>.createStepCounter(): LongArray {
            val stepsMap = values.groupingBy { it }.eachCount()

            return (0..stepsMap.keys.max()).asSequence()
                .map { stepsMap[it] ?: 0 }.chunked(2).toList()
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
