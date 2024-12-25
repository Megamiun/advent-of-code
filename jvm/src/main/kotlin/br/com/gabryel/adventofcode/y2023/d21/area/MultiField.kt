package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import java.util.*
import kotlin.math.absoluteValue
import kotlin.math.max

data class MultiField(
    override val context: Context,
    private val matrix: Map<Coordinate, AreaState>,
    override val level: Int
): Area {
    private val signalsCache = EnumMap<Direction, List<StepState>>(Direction::class.java)
    private val signalTimesCache = EnumMap<Direction, Long>(Direction::class.java)
    private val stepsCache = mutableMapOf<Long, Long>()

    companion object {
        fun growFrom(areas: Map<Coordinate, AreaState>): Area {
            val reference = areas.firstNotNullOf { it.value.second }
            val matrix = areas.mapValues { (_, data) -> data.first to data.second }
                .toMutableMap()

            val toVisit = PriorityQueue(compareBy<Pair<Coordinate, Direction>> { (coord, dir) ->
                matrix[coord]!!.first + matrix[coord]!!.second.getTimeToSignal(dir)
            })

            toVisit += Direction.entries.flatMap { dir -> matrix.keys.map { coord -> coord to dir } }

            while (toVisit.isNotEmpty()) {
                val (tile, direction) = toVisit.remove()
                val newTile = tile + direction.vector

                if (max(newTile.x().absoluteValue, newTile.y().absoluteValue) > 1) {
                    continue
                }

                if (newTile !in matrix) {
                    val (prevDistance, prevArea) = matrix[tile]!!
                    val next = prevArea.expand(direction)

                    matrix[newTile] = (prevDistance + prevArea.getTimeToSignal(direction)) to next

                    toVisit += Direction.entries.map { newTile to it }
                }
            }

            val minimal = matrix
                .filterKeys { max(it.x().absoluteValue, it.y().absoluteValue) <= 1 }
                .values.minOf { it.first }

            val filterExternal = matrix
                .filterKeys { max(it.x().absoluteValue, it.y().absoluteValue) <= 1 }
                .mapValues { (_, v) -> v.first - minimal to v.second }

            return MultiField(reference.context, filterExternal, reference.level + 1)
        }
    }

    override val stepsToEnd = matrix.values.maxOf { (k, v) -> k + v.stepsToEnd }

    override val stepsPerParity = listOf(true, false).associateWith(this::countForParity)

    override val firstOut = Direction.entries.minOf(::getTimeToSignal)

    override fun getSignals(direction: Direction) = signalsCache.getOrPut(direction) {
        filterAreas(direction).flatMap { (coord, info) ->
            info.second.getSignals(direction)
                .map { signal -> (info.first + signal.first) to (coord * context.getLevelMultiplier(level)) + signal.second }
        }
    }

    override fun getAreas(direction: Direction) = filterAreas(direction)

    override fun getTimeToSignal(direction: Direction) = signalTimesCache.getOrPut(direction) {
        filterAreas(direction)
            .minOf { (_, info) -> info.first + info.second.getTimeToSignal(direction) }
    }

    override fun expand(direction: Direction) = context.get(this, direction) {
        growFrom(getAreas(direction).mapKeys { (k) -> k + (direction.inverse().vector * 3) })
    }

    override fun afterSteps(steps: Long) = stepsCache.getOrPut(steps) {
        matrix.values.sumOf { (start, cell) -> cell.afterSteps(steps - start) }
    }

    private fun countForParity(even: Boolean) =
        matrix.values.sumOf { (distance, cell) -> cell.stepsPerParity[(distance % 2 == 0L) == even]!! }

    private fun filterAreas(direction: Direction) = matrix
        .filter { (coord) -> direction.vector.x() in listOf(0, coord.x()) && direction.vector.y() in listOf(0, coord.y()) }

    override fun toString() = "[level: $level, stepsToEnd: $stepsToEnd]"
}