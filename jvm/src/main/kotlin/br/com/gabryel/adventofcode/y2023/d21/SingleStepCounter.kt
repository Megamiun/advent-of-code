package br.com.gabryel.adventofcode.y2023.d21

import br.com.gabryel.adventofcode.y2023.d10.Coordinate
import br.com.gabryel.adventofcode.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val map = readLines(2023, 21, file)

        SingleStepCounter(map)
            .printSequenceForSteps(file, "Limited", 6, 64)

//        StepCounter(map) { mapToMap(map) }
//            .printSequenceForSteps(file, "Infinite", 1, 2, 3, 4, 5, 6, 10, 50, 64, 100, 500, 1000, 5000, 26501365)
    }
}

private fun SingleStepCounter.printSequenceForSteps(file: String, type: String, vararg printable: Int) {
    printable.forEach { steps ->
        println("[Walkable Tiles - $type][$file][$steps] ${getPossibleTilesOn(steps)}")
    }
}

private class SingleStepCounter(private val map: List<String>) {

    private val mapByDistance by lazy { loadMapByDistance() }

    fun getPossibleTilesOn(steps: Int): Long {
        val stepsParity = steps % 2
        return mapByDistance.sumOf { row -> row.count { it < steps && it % 2 == stepsParity }.toLong() }
    }

    private fun loadMapByDistance(): Array<IntArray> {
        val start = map.withIndex().firstNotNullOf { (rowIndex, row) ->
            row.withIndex().firstOrNull { it.value == 'S' }?.let { rowIndex to it.index }
        }

        val mapByDistance = Array(map.size) { IntArray(map.first().length) { Int.MAX_VALUE } }
        mapByDistance[start.first][start.second] = 0

        val toVisit = ArrayDeque<Coordinate>().apply {
            add(start)
        }

        while (toVisit.isNotEmpty()) {
            val tile = toVisit.removeFirst()
            val newDistance = mapByDistance[tile.first][tile.second] + 1

            tile.findAdjacent().forEach { (newRow, newCol) ->
                mapByDistance[newRow][newCol] = minOf(mapByDistance[newRow][newCol], newDistance)
                toVisit.add(newRow to newCol)
            }
        }
        return mapByDistance
    }

    private fun Coordinate.findAdjacent() =
        listOf(1 to 0, -1 to 0, 0 to 1, 0 to -1)
            .filter { (rowDiff, colDiff) -> getTileValue(first + rowDiff, second + colDiff) != '#' }
            .map { (rowDiff, colDiff) ->  first + rowDiff to second + colDiff }

    private fun getTileValue(row: Int, col: Int): Char {
        val (newRow, newCol) = (row to col)
        return map.getOrNull(newRow)?.getOrNull(newCol) ?: '#'
    }
}
