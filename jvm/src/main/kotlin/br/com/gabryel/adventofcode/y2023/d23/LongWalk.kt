package br.com.gabryel.adventofcode.y2023.d23

import br.com.gabryel.adventofcode.y2023.d10.Coordinate
import br.com.gabryel.adventofcode.y2023.d23.Direction.*
import br.com.gabryel.adventofcode.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(2023, 23, file)

        println("[Longest Road - Wet][$file] ${LongWalk(lines, false).findLongestPath()}")
        println("[Longest Road - Dry][$file] ${LongWalk(lines, true).findLongestPath()}")
    }
}

private class LongWalk(private val lines: List<String>, private val canClimbSlopes: Boolean) {

    private val start = 0 to 1
    private val end = lines.lastIndex to lines.first().lastIndex - 1

    private val nodeMap = lines.createNodeMap()

    fun findLongestPath() = findLongestRouteToEndFrom(0 to 1)

    private fun findLongestRouteToEndFrom(from: Coordinate, visited: List<Coordinate> = emptyList()): Int {
        if (from == end) return 0
        if (from in visited) return Int.MIN_VALUE

        val visitedWithFrom = visited + from

        return nodeMap[from]!!.maxOf { (nextNode, distance) ->
            distance + findLongestRouteToEndFrom(nextNode, visitedWithFrom)
        }
    }

    private fun List<String>.createNodeMap(): Map<Coordinate, List<Pair<Coordinate, Int>>> {
        val intersections = flatMapIndexed { rowNum, row ->
            row.indices
                .map { colNum -> rowNum to colNum }
                .filter { (row, col) -> this[row][col] == '.' }
                .filter { coord -> coord.getAdjacentNonWalls().count() > 2 }
        } + listOf(start, end)

        return intersections.createMapOfAdjacencies()
    }

    private fun List<Pair<Int, Int>>.createMapOfAdjacencies() = associateWith { root ->
        root.getMovableTiles().flatMap { adjacent ->
            val movableTo = mutableSetOf<Pair<Coordinate, Int>>()
            val visited = mutableSetOf<Coordinate>()
                .apply { add(root) }

            val toVisit = ArrayDeque<Pair<Coordinate, Int>>()
                .apply { add(adjacent to 1) }

            while (toVisit.isNotEmpty()) {
                val (current, distance) = toVisit.removeFirst()

                if (current in visited) continue
                visited.add(current)

                if (current in this) {
                    movableTo.add(current to distance)
                    continue
                }

                toVisit.addAll(current.getMovableTiles().map { it to distance + 1 })
            }
            movableTo
        }
    }

    private fun Coordinate.getAdjacentNonWalls() = Direction.entries.mapNotNull { getIfNonWall(it) }

    private fun Coordinate.getMovableTiles(): List<Pair<Int, Int>> {
        if (canClimbSlopes) return getAdjacentNonWalls()

        val (row, col) = this
        return when (lines[row][col]) {
            '.' -> getAdjacentNonWalls()
            '>' -> inDirectionIfMovable(RIGHT)
            '<' -> inDirectionIfMovable(LEFT)
            'v' -> inDirectionIfMovable(DOWN)
            '^' -> inDirectionIfMovable(UP)
            else -> emptyList()
        }
    }

    private fun Coordinate.inDirectionIfMovable(direction: Direction) =
        getIfNonWall(direction)
            ?.takeIf { (row, col) -> canClimbSlopes || getSafeTile(row, col) != direction.inverseSlope }
            ?.let { listOf(it) }
            .orEmpty()

    private fun Coordinate.getIfNonWall(direction: Direction): Pair<Int, Int>? {
        val (rowDiff, colDiff) = direction.vector
        return let { (row, col) -> row + rowDiff to col + colDiff }
            .takeIf { (row, col) -> getSafeTile(row, col) != '#' }
    }

    private fun getSafeTile(row: Int, col: Int) = lines.getOrNull(row)?.getOrNull(col) ?: '#'
}

private enum class Direction(val vector: Pair<Int, Int>, val inverseSlope: Char) {
    RIGHT(0 to 1, '>'),
    DOWN(1 to 0, 'v'),
    LEFT(0 to -1, '<'),
    UP(-1 to 0, '^');
}
