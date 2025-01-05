package br.com.gabryel.adventofcode.y2023.d10

import br.com.gabryel.adventofcode.util.*

private val inflated = mapOf(
    'F' to listOf("   ", " F-", " | "),
    'L' to listOf(" | ", " L-", "   "),
    '7' to listOf("   ", "-7 ", " | "),
    'J' to listOf(" | ", "-J ", "   "),
    '-' to listOf("   ", "---", "   "),
    '|' to listOf(" | ", " | ", " | "),
    'S' to listOf(" S ", "SSS", " S "),
    '.' to listOf("...", "...", "..."),
).mapValues { it.value.map { it.toList() } }

fun countPipeMazeInternalPositions(lines: List<String>): Int {
    val baseMap = lines.map { it.toCharArray() }.toTypedArray()

    val inflatedMap = baseMap.getInflatedMap()
    inflatedMap.markExternal()

    return baseMap.indices.sumOf { originalY ->
        baseMap.first().indices.count { originalX ->
            inflatedMap.isExternal(originalX to originalY)
        }
    }
}

private fun CharArray2D.getInflatedMap() = flatMap {
    val inflatedLine = it.map { inflated[it]!! }

    listOf(
        inflatedLine.flatMap { it[0] },
        inflatedLine.flatMap { it[1] },
        inflatedLine.flatMap { it[2] },
    )
}.map { it.toCharArray() }.toTypedArray()

private fun CharArray2D.markExternal() {
    val toVisit = getInitialQueue()

    while (toVisit.isNotEmpty()) {
        val current = toVisit.removeFirst()
        if (isWall(current)) continue

        this[current] = '0'

        for ((next) in findAdjacent(current))
            toVisit += next
    }
}

private fun CharArray2D.getInitialQueue() = ArrayDeque<Coordinate>().also { queue ->
    val maxX = first().lastIndex
    val maxY = lastIndex

    for (x in 0..maxX) {
        queue.add(x to 0)
        queue.add(x to maxY)
    }
    for (y in 0..maxY) {
        queue.add(0 to y)
        queue.add(maxX to y)
    }
}

private fun CharArray2D.isWall(current: Coordinate): Boolean {
    val currentSpace = getOrNull(current) ?: return true
    return currentSpace != '.' && currentSpace != ' '
}

private fun CharArray2D.isExternal(current: Coordinate) =
    inflateCoordinates(current)
        .none { getOrNull(it) == '0' }

private fun inflateCoordinates(current: Coordinate): List<Coordinate> {
    val baseX = current.x() * 3
    val baseY = current.y() * 3

    return listOf(0, 1, 2).flatMap { xDiff ->
        listOf(0, 1, 2).map { yDiff -> baseX + xDiff to baseY + yDiff }
    }
}
