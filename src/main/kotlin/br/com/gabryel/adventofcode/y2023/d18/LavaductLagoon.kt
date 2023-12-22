package br.com.gabryel.adventofcode.y2023.d18

import br.com.gabryel.adventofcode.y2023.d18.Direction.*
import br.com.gabryel.adventofcode.y2023.logTimed
import br.com.gabryel.adventofcode.y2023.readLines
import br.com.gabryel.adventofcode.y2023.size
import java.util.HexFormat.fromHexDigitsToLong

private enum class Direction {
    RIGHT, DOWN, LEFT, UP;

    companion object {
        fun findByInitial(initial: String) = entries.first { it.name.startsWith(initial) }
        fun findByCode(code: Int) = entries[code]
    }
}

private val directionMap = mapOf(
    LEFT to (-1L to 0L),
    RIGHT to (1L to 0L),
    UP to (0L to -1L),
    DOWN to (0L to 1L)
)

private val extractor = """(.) (\d+) \(#(.*)\)""".toRegex()

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(18, file).map {
            val (_, direction, steps, color) = extractor.find(it)!!.groupValues

            Triple(Direction.findByInitial(direction), steps.toInt(), color)
        }

        println("[Lava Area - Default][$file] ${lines.findDefaultLagoonArea()}")
        println("[Lava Area - Color  ][$file] ${lines.findColorLagoonArea()}")
    }
}

private fun List<Triple<Direction, Int, String>>.findDefaultLagoonArea() =
    map { (direction, steps) -> steps.toLong() to direction }
        .findLagoonSpace()

private fun List<Triple<Direction, Int, String>>.findColorLagoonArea() =
    map { (_, _, color) ->
        val distanceHex = fromHexDigitsToLong(color.substring(0, 5))
        val direction = Direction.findByCode(color.last().digitToInt())

        distanceHex to direction
    }.findLagoonSpace()

private fun List<Pair<Long, Direction>>.findLagoonSpace(): Long {
    return EnclosedSpacesFinder(this).countLagoonSpaces()
}

private class EnclosedSpacesFinder(instructions: List<Pair<Long, Direction>>) {
    private data class LineReport(val inside: List<LongRange> = emptyList(), val border: List<LongRange> = emptyList())

    private val borders = instructions.getBorders()
    private val sequenceStep = 3000000

    fun countLagoonSpaces() = borders.fold(LineReport() to 0L) { (previous, total), rowBorders ->
        val inside = rowBorders
            .windowed(2) { (l, r) -> l.last + 1 until r.first }
            .clearOutside(previous)

        val nonClosingBorders = excludeClosingBorders(rowBorders, previous)
        val newPoints = inside.sumOf { it.size() } + rowBorders.sumOf { it.size() }

        LineReport(inside, nonClosingBorders) to (total + newPoints)
    }.second

    private fun List<LongRange>.clearOutside(previous: LineReport) =
        filter { xs -> previous.border.any { it.intersects(xs) } || previous.inside.any { it.intersects(xs) } }

    private fun excludeClosingBorders(border: List<LongRange>, previous: LineReport) =
        border.filter { maybeClosing -> previous.inside.none { it.intersects(maybeClosing) } }

    private fun List<Pair<Long, Direction>>.getBorders(): Sequence<List<LongRange>> {
        val (min, max) = getMinMax()

        return generateSequence(min) { it + sequenceStep }
            .takeWhile { it <= max }
            .flatMap { start ->
                val range = start until start + sequenceStep
                logTimed("Starting [${range.first}, ${range.last}]")

                val byRow = drawMap()
                    .filter { it.second in range }
                    .groupBy({ it.second }) { it.first }
                    .mapValues { (_, values) -> values.findSequentialRanges() }

                byRow.toSortedMap().values
        }
    }

    private fun List<Pair<Long, Direction>>.getMinMax(): Pair<Long, Long> {
        var min = Long.MAX_VALUE
        var max = Long.MIN_VALUE

        drawMap().forEach {
            min = minOf(min, it.second)
            max = maxOf(max, it.second)
        }
        return min to max
    }

    private fun List<Pair<Long, Direction>>.drawMap() = asSequence()
        .flatMap { (steps, direction) ->
            val diff = directionMap[direction]!!
            (0 until steps).map { diff }
        }.runningFold(0L to 0L) { (x, y), (xDiff, yDiff) -> (x + xDiff) to (y + yDiff) }

    private fun List<Long>.findSequentialRanges(): List<LongRange> {
        val sorted = sorted().distinct()
        val sequenceStartingIndices = sorted.windowed(2).mapIndexed { index, (previous, current) ->
            if (previous != current - 1) index + 1
            else null
        }.filterNotNull()

        val rangesIndices = (listOf(0) + sequenceStartingIndices + size)
            .windowed(2) { (l, r) -> l to r }

        return rangesIndices.map { (start, end) -> this[start] .. this[end - 1] }
    }

    private fun LongRange.intersects(other: LongRange) =
        first in other || last in other || other.first in this || other.last in this
}
