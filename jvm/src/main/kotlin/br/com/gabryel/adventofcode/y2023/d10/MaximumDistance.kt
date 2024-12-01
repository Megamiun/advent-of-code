package br.com.gabryel.adventofcode.y2023.d10

private val leftToRight = "SFL-".toSet() to "S7J-".toSet()
private val topToBottom = "SF7|".toSet() to "SLJ|".toSet()

private val rightToLeft = leftToRight.let { (first, second) -> second to first }
private val bottomToTop = topToBottom.let { (first, second) -> second to first }

private typealias ToFromPossibilities = Pair<Set<Char>, Set<Char>>

class MaximumDistance(private val map: CoordMap) {
    private val distances = mutableMapOf<Coordinate, Int>()

    private val toVisit = ArrayDeque<Pair<Int, Coordinate>>()

    val maxDistance by lazy { map.findMaxDistance() }

    private fun CoordMap.findMaxDistance(): Int {
        val (startX, startY) = mapIndexed { index, line -> index to line.indexOfFirst { it == 'S' } }
            .first { it.second != -1 }

        toVisit.add(0 to (startX to startY))

        while (toVisit.isNotEmpty()) {
            val (distance, coordinate) = toVisit.removeFirst()
            if (distances[coordinate] != null) continue

            distances[coordinate] = distance
            addAdjacentsToVisit(coordinate, distance)
        }

        return distances.maxOf { it.value }
    }

    private fun addAdjacentsToVisit(current: Coordinate, distance: Int) {
        val currentSpace = map.findOnPosition(current)
        val (currentX, currentY) = current

        addToVisitIfConnected(currentX to currentY + 1, currentSpace, distance, leftToRight)
        addToVisitIfConnected(currentX to currentY - 1, currentSpace, distance, rightToLeft)
        addToVisitIfConnected(currentX + 1 to currentY, currentSpace, distance, topToBottom)
        addToVisitIfConnected(currentX - 1 to currentY, currentSpace, distance, bottomToTop)
    }

    private fun addToVisitIfConnected(
        nextCoordinate: Pair<Int, Int>,
        currentSpace: Char,
        distance: Int,
        toFrom: ToFromPossibilities
    ) {
        val (to, from) = toFrom
        if (currentSpace !in to) return

        val right = map.findOnPosition(nextCoordinate)
        if (right in from) toVisit.add(distance + 1 to nextCoordinate)
    }
}