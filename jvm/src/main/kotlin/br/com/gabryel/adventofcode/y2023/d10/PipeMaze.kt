package br.com.gabryel.adventofcode.y2023.d10

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.util.Direction.*
import java.util.*
import kotlin.collections.ArrayDeque
import kotlin.collections.set

private val directions = EnumMap(
    mapOf(
        LEFT to "SFL-",
        RIGHT to "S7J-",
        UP to "SF7|",
        DOWN to "SLJ|",
    )
)

fun getPipeMazeMaximumDistance(lines: List<String>) =
    lines.map { it.toCharArray() }.toTypedArray().findMaxDistance()

private fun CharArray2D.findMaxDistance(): Int {
    val distances = mutableMapOf<Coordinate, Int>()
    val toVisit = ArrayDeque<Pair<Int, Coordinate>>()

    toVisit.add(0 to findFirst('S'))

    while (toVisit.isNotEmpty()) {
        val (distance, coordinate) = toVisit.removeFirst()
        if (coordinate in distances) continue

        distances[coordinate] = distance
        val currentSpace = this[coordinate]

        for (direction in Direction.entries) {
            if (currentSpace !in directions[direction.inverse()]!!) continue

            val nextCoordinate = coordinate + direction
            val nextContent = this.getOrNull(nextCoordinate) ?: continue

            if (nextContent in directions[direction]!!)
                toVisit.add(distance + 1 to nextCoordinate)
        }
    }

    return distances.maxOf { it.value }
}
