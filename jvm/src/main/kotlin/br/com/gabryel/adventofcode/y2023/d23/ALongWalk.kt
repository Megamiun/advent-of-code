package br.com.gabryel.adventofcode.y2023.d23

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.util.Direction.*

fun findLongestWalk(lines: List<String>, canClimbSlopes: Boolean) = LongWalk(lines, canClimbSlopes)
    .findLongestPath()

private class LongWalk(lines: List<String>, private val canClimbSlopes: Boolean) {

    private val start = 1 to 0

    private val end = lines.lastIndex - 1 to lines.first().lastIndex

    private val map = lines.toCharArray2D()

    private val nodeMap = array2D<List<Pair<Coordinate, Int>>>(lines.size, lines[0].length).apply {
        populate(start)
    }

    fun findLongestPath() = findLongestRouteToEndFrom(1 to 0)

    private fun findLongestRouteToEndFrom(
        from: Coordinate,
        visited: BooleanArray2D = Array(nodeMap.size) { BooleanArray(nodeMap[0].size) }
    ): Int {
        if (from in visited) return Int.MIN_VALUE
        if (from == end) return 0

        visited[from] = true

        val result = nodeMap[from]!!.maxOf { (nextNode, distance) ->
            distance + findLongestRouteToEndFrom(nextNode, visited)
        }

        visited[from] = false
        return result
    }

    private fun Array2D<List<Pair<Coordinate, Int>>?>.populate(start: Coordinate) {
        if (start in this) return

        val adjacent = start.getAdjacentMovable()
            .map { (position, _, direction) -> position.findNextIntersection(direction) }

        this[start] = adjacent
        for ((next) in adjacent)
            populate(next)
    }

    private tailrec fun Coordinate.findNextIntersection(
        direction: Direction,
        distance: Int = 1
    ): Pair<Coordinate, Int> {
        val inverse = direction.inverse()
        val movable = getAdjacentMovable().filter { it.direction() != inverse }

        if (movable.size == 1) {
            val (curr, _, dir) = movable.single()
            return curr.findNextIntersection(dir, distance + 1)
        }

        return this to distance
    }

    private fun Coordinate.getAdjacentMovable() =
        map.findAdjacent(this).filter { it.isMovableFrom() }

    private fun Adjacency<Char>.isMovableFrom(): Boolean {
        val content = content()
        if (content == '#') return false
        return content == '.' || canClimbSlopes || content == direction().slope()
    }
}

private fun Direction.slope() = when (this) {
    RIGHT -> '>'
    DOWN -> 'v'
    LEFT -> '<'
    UP -> '^'
}
