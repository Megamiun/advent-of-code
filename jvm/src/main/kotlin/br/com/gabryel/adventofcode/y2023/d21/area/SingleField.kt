package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import br.com.gabryel.adventofcode.y2023.d21.area.CellType.GROUND
import br.com.gabryel.adventofcode.y2023.d21.area.CellType.OUTSIDE
import java.util.EnumMap

data class SingleField(
    override val context: Context,
    private val starts: List<StepState>,
    private val signals: Map<Direction, List<StepState>>,
    private val distanceMap: Map<Coordinate, Long>
) : Area {

    override val level = 1

    override val stepsToEnd = distanceMap.values.max()

    override val stepsPerParity = listOf(true, false).associateWith(this::countForParity)

    override val firstOut = signals.values.minOf { it.minOf { it.first } }

    override fun afterSteps(steps: Long) =
        if (steps < stepsToEnd)
            distanceMap.values.count { distance -> distance <= steps && distance % 2 == steps % 2 }.toLong()
        else
            stepsPerParity[steps % 2 == 0L]!!

    override fun getSignals(direction: Direction) = signals[direction]!!

    override fun getAreas(direction: Direction) = mapOf((0 to 0) to (0L to this))

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

            return SingleField(context, starts, signals, distances)
        }

        private fun Char?.getType(): CellType {
            return when (this) {
                '#' -> CellType.WALL
                null -> OUTSIDE
                else -> GROUND
            }
        }
    }

    private fun countForParity(even: Boolean) =
        distanceMap.values.count { distance -> (distance % 2 == 0L) == even }.toLong()

    override fun toString() = "[level 1, stepsToEnd: $stepsToEnd, hash: ${System.identityHashCode(this)}]"
}

private enum class CellType {
    WALL, GROUND, OUTSIDE
}
