package br.com.gabryel.adventofcode.y2024.d06

import br.com.gabryel.adventofcode.util.Coordinate
import br.com.gabryel.adventofcode.util.plus
import br.com.gabryel.adventofcode.util.x
import br.com.gabryel.adventofcode.util.y
import br.com.gabryel.adventofcode.util.readLines
import java.util.HashSet

enum class Direction(val diff: Coordinate, val next: () -> Direction, val character: Char) {
    N(0 to -1, { E }, '^'),
    E(1 to 0, { S }, '>'),
    S(0 to 1, { W }, 'V'),
    W(-1 to 0, { N }, '<');
}

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(2024, 6, file)
            .map { it.toCharArray().toList() }

        println("[Visited][$file] ${lines.getVisitedCount()}")
        println("[Loops  ][$file] ${lines.getLoopsAfterObstacle()}")
    }
}

fun List<List<Char>>.getVisitedCount(): Int {
    val guard = Direction.entries
        .firstNotNullOf { dir -> findCharacter(dir.character)?.let { it to dir } }

    return generateSequence(guard) { (position, direction) -> visitNext(position, direction) }
        .distinctBy { (pos) -> pos }
        .count()
}

fun List<List<Char>>.getLoopsAfterObstacle(): Int {
    val guard = Direction.entries
        .firstNotNullOf { dir -> findCharacter(dir.character)?.let { it to dir } }

    val visitableByGuard = generateSequence(guard) { (position, direction) -> visitNext(position, direction) }
        .map { (pos) -> pos }
        .distinct()

    return visitableByGuard.count { causesLoop(it, guard) }
}

fun List<List<Char>>.causesLoop(obstacle: Coordinate, guard: Pair<Coordinate, Direction>): Boolean {
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

fun List<List<Char>>.visitNextWithObstacle(position: Coordinate, direction: Direction, obstacle: Coordinate): Pair<Coordinate, Direction>? {
    val next = position + direction.diff

    return if (next == obstacle) {
        position to direction.next()
    } else when (getAtSafe(next)) {
        '#' -> position to direction.next()
        null -> null
        else -> next to direction
    }
}

fun List<List<Char>>.visitNext(position: Coordinate, direction: Direction): Pair<Coordinate, Direction>? {
    val next = position + direction.diff
    return when (getAtSafe(next)) {
        '#' -> position to direction.next()
        null -> null
        else -> next to direction
    }
}

private fun List<List<Char>>.findCharacter(searched: Char): Coordinate? =
    findAll(searched).firstOrNull()

private fun List<List<Char>>.findAll(searched: Char) =
    withIndex().asSequence().flatMap { (y, line) ->
        line.withIndex()
            .asSequence()
            .filter { (_, character) -> character == searched }
            .map { (x) -> x to y }
    }

private fun List<List<Char>>.getAtSafe(position: Coordinate) =
    getOrNull(position.y())?.getOrNull(position.x())

private fun Pair<*, Direction>.getDirection() = second
