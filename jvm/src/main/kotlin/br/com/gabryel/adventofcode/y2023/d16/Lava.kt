package br.com.gabryel.adventofcode.y2023.d16

import br.com.gabryel.adventofcode.y2023.d16.Direction.*
import br.com.gabryel.adventofcode.util.readLines

private typealias DirectionsMap = Array<Array<MutableSet<Direction>>>

private enum class Direction { UP, DOWN, RIGHT, LEFT }

private val movementMap = mapOf(
    '.' to mapOf(LEFT to listOf(LEFT), UP to listOf(UP), RIGHT to listOf(RIGHT), DOWN to listOf(DOWN)),
    '\\' to mapOf(LEFT to listOf(UP), UP to listOf(LEFT), RIGHT to listOf(DOWN), DOWN to listOf(RIGHT)),
    '/' to mapOf(LEFT to listOf(DOWN), UP to listOf(RIGHT), RIGHT to listOf(UP), DOWN to listOf(LEFT)),
    '-' to mapOf(LEFT to listOf(LEFT), UP to listOf(LEFT, RIGHT), RIGHT to listOf(RIGHT), DOWN to listOf(LEFT, RIGHT)),
    '|' to mapOf(LEFT to listOf(UP, DOWN), UP to listOf(UP), RIGHT to listOf(UP, DOWN), DOWN to listOf(DOWN))
)

fun main() {
    listOf("sample", "input").forEach { file ->
        val instructions = readLines(2023, 16, file)

        println("[Heated Tiles    ][$file] ${instructions.getEnergizedTiles(-1, 0, RIGHT)}")
        println("[Heated Tiles Max][$file] ${instructions.getMaxEnergizedTiles()}")
    }
}

private fun List<String>.getMaxEnergizedTiles(): Int {
    val upDown = 0..first().lastIndex
    val leftRight = 0..lastIndex
    val allStartPoints = mapOf(
        RIGHT to leftRight.map { -1 to it },
        LEFT to leftRight.map { first().length  to it},
        UP to upDown.map { it to size },
        DOWN to upDown.map { it to -1 },
    )

    return allStartPoints.maxOf { (direction, startingPoints) ->
        startingPoints.maxOf { (x, y) -> getEnergizedTiles(x, y, direction) }
    }
}

private fun List<String>.getEnergizedTiles(x: Int, y: Int, direction: Direction): Int {
    val moved = Array(size) { Array(first().length) { mutableSetOf<Direction>() } }

    moveFrom(x, y, direction, moved)

    return moved.sumOf { it.count { it.isNotEmpty() } }
}

private tailrec fun List<String>.moveFrom(prevX: Int, prevY: Int, direction: Direction, moved: DirectionsMap) {
    val (x, y) = findPositionAfterMovement(direction, prevX, prevY)

    val currentTile = getOrNull(y)?.getOrNull(x)
        ?: return

    val spentDirections = moved[x][y]
    if (direction in spentDirections) return
    spentDirections.add(direction)

    val nextMovements = movementMap[currentTile]!![direction]!!

    nextMovements.dropLast(1).forEach { newDirection ->
        moveFrom(x, y, newDirection, moved)
    }

    // To avoid Stack Overflows using tailrec
    moveFrom(x, y, nextMovements.last(), moved)
}

private fun findPositionAfterMovement(direction: Direction, x: Int, y: Int) = when (direction) {
    UP -> x to y - 1
    DOWN -> x to y + 1
    RIGHT -> x + 1 to y
    LEFT -> x - 1 to y
}
