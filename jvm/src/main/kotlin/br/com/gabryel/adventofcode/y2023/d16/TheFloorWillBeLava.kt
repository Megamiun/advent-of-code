package br.com.gabryel.adventofcode.y2023.d16

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.util.Direction.*
import java.util.*

private typealias VisitationMap = Array<Array<EnumSet<Direction>>>

private val movementMap = mapOf(
    '\\' to mapOf(LEFT to listOf(UP), UP to listOf(LEFT), RIGHT to listOf(DOWN), DOWN to listOf(RIGHT)),
    '/' to mapOf(LEFT to listOf(DOWN), UP to listOf(RIGHT), RIGHT to listOf(UP), DOWN to listOf(LEFT)),
    '-' to mapOf(LEFT to listOf(LEFT), UP to listOf(LEFT, RIGHT), RIGHT to listOf(RIGHT), DOWN to listOf(LEFT, RIGHT)),
    '|' to mapOf(LEFT to listOf(UP, DOWN), UP to listOf(UP), RIGHT to listOf(UP, DOWN), DOWN to listOf(DOWN))
)

fun getMaxEnergizedTiles(lines: List<String>): Int {
    val upDown = 0..lines.first().lastIndex
    val leftRight = 0..lines.lastIndex
    val allStartPoints = mapOf(
        RIGHT to leftRight.map { -1 to it },
        LEFT to leftRight.map { lines.first().length  to it},
        UP to upDown.map { it to lines.size },
        DOWN to upDown.map { it to -1 },
    )

    val map = lines.toCharArray2D()

    return allStartPoints.maxOf { (direction, startingPoints) ->
        startingPoints.maxOf { curr -> map.getEnergizedTiles(curr, direction) }
    }
}

fun getEnergizedTiles(lines: List<String>, current: Coordinate = -1 to 0, direction: Direction = RIGHT) =
    lines.toCharArray2D().getEnergizedTiles(current, direction)

private fun CharArray2D.getEnergizedTiles(current: Coordinate, direction: Direction): Int {
    val visitations = Array(size) { Array(first().size) { EnumSet.noneOf(Direction::class.java) } }

    moveFrom(current, direction, visitations)

    return visitations.sumOf { it.count { it.isNotEmpty() } }
}

private tailrec fun CharArray2D.moveFrom(previous: Coordinate, direction: Direction, visitations: VisitationMap) {
    val curr = previous + direction
    val currentTile = getOrNull(curr) ?: return

    if (!visitations[curr].add(direction))
        return

    if (currentTile != '.') {
        val nextMovements = movementMap[currentTile]!![direction]!!

        return nextMovements.forEach { newDirection ->
            moveFrom(curr, newDirection, visitations)
        }
    }

    return moveFrom(curr, direction, visitations)
}
