package br.com.gabryel.adventofcode.y2023.d14

import br.com.gabryel.adventofcode.util.CharArray2D
import br.com.gabryel.adventofcode.util.toCharArray2D

fun calculateStressAfterSingleMovement(lines: List<String>) =
    StressCalculator(lines).getStressAfterSingleMovement()

fun calculateStress(lines: List<String>, rotations: Int) =
    StressCalculator(lines).getStressAfterRotations(rotations)

private class StressCalculator(private val terrain: List<String>) {
    private val rows = terrain.indices

    private val columns = terrain.first().indices

    private val betweenCubeRocksRowRanges = terrain.getSquareRocksPerRow().getRanges(rows.last + 1)

    private val betweenCubeRocksColRanges = terrain.getSquareRocksPerColumn().getRanges(columns.last + 1)

    fun getStressAfterSingleMovement() = terrain.toCharArray2D().run {
        moveVertical(down = false)
        calculateStressForState()
    }

    fun getStressAfterRotations(remaining: Int) =
        terrain.toCharArray2D().getStressAfterRotations(remaining)

    private tailrec fun CharArray2D.getStressAfterRotations(
        remaining: Int,
        cache: MutableMap<List<List<Char>>, Int> = mutableMapOf()
    ): Int {
        if (remaining <= 0L) return calculateStressForState()

        moveVertical(down = false)
        moveHorizontal(right = false)
        moveVertical(down = true)
        moveHorizontal(right = true)

        val currentState = map { it.toList() }
        if (cache.containsKey(currentState))
            return cache.calculateStressForCycle(currentState, remaining)

        cache[currentState] = remaining

        return getStressAfterRotations(remaining - 1, cache)
    }

    private fun Map<List<List<Char>>, Int>.calculateStressForCycle(state: List<List<Char>>, remaining: Int): Int {
        val cycleStart = this[state]!!
        val cycleLength = cycleStart - remaining
        val remainingReduced = (remaining - 1) % cycleLength
        val finalState = cycleStart - remainingReduced

        return entries
            .first { it.value == finalState }
            .key.calculateStressForState()
    }

    private fun List<List<Char>>.calculateStressForState() =
        reversed().mapIndexed { index, row -> row.count { it == 'O' } * (index + 1) }.sum()

    private fun CharArray2D.calculateStressForState() =
        reversed().mapIndexed { index, row -> row.count { it == 'O' } * (index + 1) }.sum()

    private fun CharArray2D.moveHorizontal(right: Boolean) {
        rows.forEach { rowNum ->
            val row = this[rowNum]
            betweenCubeRocksRowRanges[rowNum].forEach { range ->
                val rocks = range.count { row[it] == 'O' }
                range.forEach { row[it] = '.' }

                val indices = if (right) range.reversed() else range
                indices.take(rocks).forEach { row[it] = 'O' }
            }
        }
    }

    private fun CharArray2D.moveVertical(down: Boolean) {
        columns.forEach { colNum ->
            betweenCubeRocksColRanges[colNum].forEach { range ->
                val rocks = range.count { this[it][colNum] == 'O' }
                range.forEach { this[it][colNum] = '.' }

                val indices = if (down) range.reversed() else range
                indices.take(rocks).forEach { this[it][colNum] = 'O' }
            }
        }
    }

    private fun List<List<Int>>.getRanges(last: Int) =
        map { (listOf(-1) + it + last).windowed(2) { (l, r) -> l + 1 until r } }

    private fun List<String>.getSquareRocksPerRow() = map { row ->
        row.mapIndexedNotNull { colNum, space ->
            colNum.takeIf { space == '#' }
        }
    }

    private fun List<String>.getSquareRocksPerColumn() = columns.map { colNum ->
        mapIndexedNotNull { rowNum, row ->
            rowNum.takeIf { row[colNum] == '#' }
        }
    }
}
