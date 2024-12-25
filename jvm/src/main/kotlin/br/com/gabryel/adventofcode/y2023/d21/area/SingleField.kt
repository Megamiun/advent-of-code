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
    private val distanceMap: Array<LongArray>
) : Area {

    override val level = 1

    override val stepsToEnd = distanceMap.maxOf { it.maxOf { it } }

    override val stepsPerParity = listOf(true, false).associateWith(this::countForParity)

    override val firstOut = signals.values.minOf { it.minOf { it.first } }

    private val stepsCache = mutableMapOf<Long, Long>()

    override fun afterSteps(steps: Long) = stepsCache.getOrPut(steps) {
        if (steps < stepsToEnd)
            distanceMap.sumOf {
                it.count { distance -> distance != -1L && distance <= steps && distance % 2 == steps % 2 }.toLong()
            }
        else
            stepsPerParity[steps % 2 == 0L]!!
    }

    override fun getSignals(direction: Direction) = signals[direction]!!

    override fun getTimeToSignal(direction: Direction) = getSignals(direction).minOf { it.first }

    override fun expand(direction: Direction): Area = context.get(this, direction) {
        from(context, getSignals(direction))
    }

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

            val arrayMatrix = Array(context.dimension) { y ->
                LongArray(context.dimension) { x ->
                    distances[x to y] ?: -1
                }
            }

            return SingleField(context, signals, arrayMatrix)
        }

        private fun Char?.getType(): CellType {
            return when (this) {
                '#' -> WALL
                null -> OUTSIDE
                else -> GROUND
            }
        }
    }

    private fun countForParity(even: Boolean) =
        distanceMap.sumOf { it.count { distance -> distance != -1L && ((distance % 2 == 0L) == even) }.toLong() }

    override fun toString() = "[level 1, stepsToEnd: $stepsToEnd, hash: ${System.identityHashCode(this)}]"
}

private enum class CellType {
    WALL, GROUND, OUTSIDE
}
