package br.com.gabryel.adventofcode.y2023.d10

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

fun countInternalPositions(baseMap: CoordMap): Int {
    val inflatedMap = baseMap.getInflatedMap()
    inflatedMap.markExternal()

    return baseMap.indices.sumOf { originalX ->
        baseMap.first().indices.count { originalY ->
            inflatedMap.isExternal(originalX, originalY)
        }
    }
}

private fun CoordMap.getInflatedMap() = toList().flatMap {
    val inflatedLine = it.map { inflated[it]!! }

    listOf(
        inflatedLine.flatMap { it[0] },
        inflatedLine.flatMap { it[1] },
        inflatedLine.flatMap { it[2] },
    )
}.map { it.toCharArray() }.toTypedArray()

private fun CoordMap.markExternal() {
    val maxX = lastIndex
    val maxY = first().lastIndex

    val toVisit = ArrayDeque<Coordinate>().apply {
        (0 .. maxX).forEach { x ->
            add(x to 0)
            add(x to maxY)
        }
        (0 .. maxY).forEach { y ->
            add(0 to y)
            add(maxX to y)
        }
    }

    while (toVisit.isNotEmpty()) {
        val current = toVisit.removeFirst()
        if (isWall(current)) continue

        setOnPosition(current, '0')
        val (x, y) = current

        toVisit.add(x to y + 1)
        toVisit.add(x to y - 1)
        toVisit.add(x + 1 to y)
        toVisit.add(x - 1 to y)
    }
}

private fun CoordMap.isWall(current: Coordinate): Boolean {
    val currentSpace = findOnPosition(current)
    return currentSpace != '.' && currentSpace != ' '
}

private fun Array<CharArray>.isExternal(x: Int, y: Int) =
    getInflatedCoordinates(x, y)
        .map { findOnPosition(it) }.none { it == '0' }

private fun getInflatedCoordinates(upperLeftX: Int, upperLeftY: Int) =
    listOf(0, 1, 2).flatMap { xDiff ->
        listOf(0, 1, 2).map { yDiff ->
            (upperLeftX * 3) + xDiff to (upperLeftY * 3) + yDiff
        }
    }

private fun CoordMap.setOnPosition(origin: Coordinate, value: Char) {
    val line = getOrNull(origin.first) ?: return
    if (origin.second !in line.indices) return

    line[origin.second] = value
}
