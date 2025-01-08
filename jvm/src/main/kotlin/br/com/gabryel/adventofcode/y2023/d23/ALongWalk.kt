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

    private fun findLongestRouteToEndFrom(from: Coordinate, visited: MutableSet<Coordinate> = mutableSetOf()): Int {
        if (from == end) return 0
        if (from in visited) return Int.MIN_VALUE

        visited += from

        val result = nodeMap[from]!!.maxOf { (nextNode, distance) ->
            distance + findLongestRouteToEndFrom(nextNode, visited)
        }

        visited -= from
        return result
    }

    private fun Array2D<List<Pair<Coordinate, Int>>?>.populate(start: Coordinate) {
        if (start in this) return

        val adjacent = start.getAdjacentMovable()
            .mapNotNull { (position, direction) -> position.findNextIntersection(direction) }
        this[start] = adjacent

        for ((next) in adjacent)
            populate(next)
    }

    private tailrec fun Coordinate.findNextIntersection(direction: Direction, distance: Int = 1): Pair<Coordinate, Int>? {
        val inverse = direction.inverse()
        val movable = getAdjacentMovable().filter { it.second != inverse }

        if (movable.size == 1) {
            val (curr, dir) = movable.single()
            return curr.findNextIntersection(dir, distance + 1)
        }

        return this to distance
    }

    private fun Coordinate.getAdjacentMovable() = Direction.entries.mapNotNull { dir ->
        val next = this.inDirectionIfMovable(dir) ?: return@mapNotNull null
        next to dir
    }

    private fun Coordinate.inDirectionIfMovable(direction: Direction): Coordinate? {
        val next = this + direction
        val content = getSafeTile(next)

        if (content == '#') return null
        if (content == '.' || content == direction.slope() || canClimbSlopes) return next

        return null
    }

    private fun getSafeTile(position: Coordinate) = map.getOrNull(position) ?: '#'
}

private fun Direction.slope() = when (this) {
    RIGHT -> '>'
    DOWN -> 'v'
    LEFT -> '<'
    UP -> '^'
}
