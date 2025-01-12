package br.com.gabryel.adventofcode.y2024.d06

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.util.Direction.*

fun getGuardLoopsAfterObstacle(lines: List<String>) = lines.toCharArray2D().getLoopsAfterObstacle()

fun getGuardVisitedCount(lines: List<String>) = lines.toCharArray2D().getVisitedCount()

fun CharArray2D.getVisitedCount(): Int {
    val guard = Direction.entries
        .firstNotNullOf { dir -> findFirstOrNull(dir.getCharacter())?.let { it to dir } }

    return generateSequence(guard) { (position, direction) -> visitNext(position, direction) }
        .distinctBy { (pos) -> pos }
        .count()
}

fun CharArray2D.getLoopsAfterObstacle(): Int {
    val guard = Direction.entries
        .firstNotNullOf { dir -> findFirstOrNull(dir.getCharacter())?.let { it to dir } }

    val visitableByGuard = generateSequence(guard) { (position, direction) -> visitNext(position, direction) }
        .map { (pos) -> pos }
        .distinct()

    return visitableByGuard.count { causesLoop(it, guard) }
}

fun CharArray2D.causesLoop(obstacle: Coordinate, guard: Pair<Coordinate, Direction>): Boolean {
    val visited = HashSet<Pair<Coordinate, Direction>>(size * size)
    var curr = guard

    while (true) {
        val (position, direction) = curr
        val next = visitNextWithObstacle(position, direction, obstacle) ?: return false

        if (next.getDirection() != direction) {
            if (curr in visited) return true
            visited += curr
        }

        curr = next
    }
}

fun CharArray2D.visitNextWithObstacle(position: Coordinate, direction: Direction, obstacle: Coordinate): Pair<Coordinate, Direction>? {
    val next = position + direction.vector

    return if (next == obstacle) {
        position to direction.clockwise()
    } else when (getOrNull(next)) {
        '#' -> position to direction.clockwise()
        null -> null
        else -> next to direction
    }
}

fun CharArray2D.visitNext(position: Coordinate, direction: Direction): Pair<Coordinate, Direction>? {
    val next = position + direction.vector
    return when (getOrNull(next)) {
        '#' -> position to direction.clockwise()
        null -> null
        else -> next to direction
    }
}

private fun Pair<*, Direction>.getDirection() = second

private fun Direction.getCharacter() = when (this) {
    RIGHT -> '>'
    DOWN -> 'V'
    LEFT -> '<'
    UP -> '^'
}
