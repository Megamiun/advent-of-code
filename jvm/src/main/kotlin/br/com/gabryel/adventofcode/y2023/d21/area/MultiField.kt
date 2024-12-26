package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.y2023.d21.area.Area.Context
import java.util.*
import kotlin.math.absoluteValue
import kotlin.math.max

class MultiField(
    override val context: Context,
    private val matrix: Map<Coordinate, AreaState>,
    override val level: Int
) : Area {

    override val stepsToEnd = matrix.values.maxOf { (k, v) -> k + v.stepsToEnd }

    private val signalsCache = EnumMap<Direction, List<StepState>>(Direction::class.java)
    private val signalTimesCache = EnumMap<Direction, Long>(Direction::class.java)
    private val stepsCache = mutableMapOf<Long, Long>()
    private val parityCache = longArrayOf(getPossiblePerParity(0), getPossiblePerParity(1))

    override val firstSignal = Direction.entries.minOf(::getTimeToSignal)

    companion object {
        fun growFrom(areas: Map<Coordinate, AreaState>): Area {
            val reference = areas.firstNotNullOf { it.value.second }
            val context = reference.context

            val matrix = areas.mapValues { (_, data) -> data.first to data.second }.toMutableMap()

            val toVisit = PriorityQueue(compareBy<Pair<Coordinate, Direction>> { (coord, dir) ->
                matrix[coord]!!.first + matrix[coord]!!.second.getTimeToSignal(dir)
            })

            toVisit += Direction.entries.flatMap { dir -> matrix.keys.map { coord -> coord to dir } }

            while (toVisit.isNotEmpty()) {
                val (tile, direction) = toVisit.remove()
                val newTile = tile + direction.vector

                if (!newTile.isInBounds(context)) continue

                if (newTile !in matrix) {
                    val (prevDistance, prevArea) = matrix[tile]!!
                    val next = prevArea.expand(direction)

                    matrix[newTile] = (prevDistance + prevArea.getTimeToSignal(direction)) to next

                    toVisit += Direction.entries.map { newTile to it }
                }
            }

            val inBounds = matrix.filterKeys { it.isInBounds(context) }
            val minimal = inBounds.values.minOf { it.first }

            val corrected = inBounds.mapValues { (_, v) -> v.first - minimal to v.second }

            return MultiField(context, corrected, reference.level + 1)
        }

        private infix fun Coordinate.isInBounds(context: Context) =
            max(x().absoluteValue, y().absoluteValue) <= context.halfLevelFactor
    }

    override fun getSignals(direction: Direction) = signalsCache.getOrPut(direction) {
        val allSignals = filterAreas(direction).flatMap { (coord, info) ->
            val baseCoord = coord * context.getLevelMultiplier(level)

            info.second.getSignals(direction).asSequence()
                .map { signal -> (info.first + signal.first) to baseCoord + signal.second }
        }

        allSignals.filter { (lDistance, lCoord) ->
            allSignals.none { (rDistance, rCoord) ->
                lCoord != rCoord && (lCoord.getManhattanDistance(rCoord)) == (lDistance - rDistance).toInt()
            }
        }
    }

    override fun getTimeToSignal(direction: Direction) = signalTimesCache.getOrPut(direction) {
        filterAreas(direction)
            .minOf { (_, info) -> info.first + info.second.getTimeToSignal(direction) }
    }

    override fun expand(direction: Direction) = context.get(this, direction) {
        growFrom(filterAreas(direction).mapKeys { (coord) -> coord + (direction.inverse().vector * context.levelFactor) })
    }

    override fun countPossibleAtStep(steps: Long) =
        if (steps < 0) 0
        else if (steps >= stepsToEnd) parityCache[(steps % 2).toInt()]
        else countPossibleCache(steps)

    private fun getPossiblePerParity(parity: Long) =
        countPossibleCache((stepsToEnd downTo 0).first { it % 2 == parity })

    private fun countPossibleCache(steps: Long) = stepsCache.getOrPut(steps) {
        matrix.values.sumOf { (start, cell) -> cell.countPossibleAtStep(steps - start) }
    }

    private fun filterAreas(direction: Direction) = matrix.filter { (coord) ->
        direction.vector.x() * context.halfLevelFactor in listOf(0, coord.x()) && direction.vector.y() * context.halfLevelFactor in listOf(0, coord.y())
    }

    override fun toString() = "[level: $level, stepsToEnd: $stepsToEnd]"
}
