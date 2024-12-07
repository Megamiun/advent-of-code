package br.com.gabryel.adventofcode.y2023.d14

import br.com.gabryel.adventofcode.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val terrain = readLines(2023, 14, file)

        val calculator = StressCalculator(terrain)

        println("[Stress Single][$file] ${calculator.calculateStressAfterSingleMovement()}")
        println("[Stress Cyclic][$file] ${calculator.calculateStress(1000000000)}")
    }
}

private class StressCalculator(private val terrain: List<String>) {
    private val rows = terrain.indices

    private val columns = terrain.first().indices

    private val rowRocks = terrain.getPositionsPerRow('#').getRanges(rows.last + 1)

    private val colRocks = terrain.getPositionsPerColumn('#').getRanges(columns.last + 1)

    fun calculateStressAfterSingleMovement() =
        terrain.map { it.toCharArray() }.toTypedArray()
            .getStressAfterSingleMovement()

    fun calculateStress(cycles: Int) =
        terrain.map { it.toCharArray() }.toTypedArray()
            .getStress(cycles)

    private fun Array<CharArray>.getStressAfterSingleMovement(): Int {
        moveVertical(down = false)
        return calculateStressForState()
    }

    private tailrec fun Array<CharArray>.getStress(
        remaining: Int,
        cache: MutableMap<String, Int> = mutableMapOf(),
        cycleFound: Boolean = false
    ): Int {
        if (remaining <= 0L) return calculateStressForState()

        moveVertical(down = false)
        moveHorizontal(right = false)
        moveVertical(down = true)
        moveHorizontal(right = true)

        if (!cycleFound) {
            val currentState = joinToString("\n") { it.joinToString("") }
            val cached = cache[currentState]
            if (cached != null) {
                val cycleLength = cached - remaining
                return getStress((remaining % cycleLength) - 1, cache, true)
            }

            cache[currentState] = remaining
        }

        return getStress(remaining - 1, cache, cycleFound)
    }

    private fun Array<CharArray>.calculateStressForState() =
        reversed().mapIndexed { index, row -> row.count { it == 'O' } * (index + 1) }.sum()

    private fun Array<CharArray>.moveHorizontal(right: Boolean) {
        rows.forEach { rowNum ->
            val row = this[rowNum]
            rowRocks[rowNum].forEach {
                val rocks = it.count { row[it] == 'O' }
                it.forEach { row[it] = '.' }

                val indices = if (right) it.reversed() else it
                indices.take(rocks).forEach { row[it] = 'O' }
            }
        }
    }

    private fun Array<CharArray>.moveVertical(down: Boolean) {
        columns.forEach { colNum ->
            colRocks[colNum].forEach {
                val rocks = it.count { this[it][colNum] == 'O' }
                it.forEach { this[it][colNum] = '.' }

                val indices = if (down) it.reversed() else it
                indices.take(rocks).forEach { this[it][colNum] = 'O' }
            }
        }
    }

    private fun List<List<Int>>.getRanges(last: Int) =
        map { ((it + -1) + last).sorted().windowed(2) { (l, r) -> l + 1 until r } }

    private fun List<String>.getPositionsPerRow(type: Char) = map { row ->
        row.mapIndexed { colNum, space ->
            if (space == type) colNum
            else null
        }.filterNotNull()
    }

    private fun List<String>.getPositionsPerColumn(type: Char) = columns.map { colNum ->
        mapIndexedNotNull { rowNum, row ->
            if (row[colNum] == type) rowNum
            else null
        }
    }
}
