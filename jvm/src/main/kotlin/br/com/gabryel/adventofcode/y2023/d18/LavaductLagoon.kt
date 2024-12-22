package br.com.gabryel.adventofcode.y2023.d18

import br.com.gabryel.adventofcode.util.Coordinate
import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.y2023.*
import br.com.gabryel.adventofcode.y2023.d18.Direction.*
import java.util.EnumMap
import java.util.HexFormat.fromHexDigits
import kotlin.math.pow

private enum class Direction {
    RIGHT, DOWN, LEFT, UP;

    companion object {
        fun findByInitial(initial: String) = entries.first { it.name.startsWith(initial) }
        fun findByCode(code: Int) = entries[code]
    }
}

private val extractor = """(.) (\d+) \(#(.*)\)""".toRegex()

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(2023, 18, file).map {
            val (_, direction, steps, color) = extractor.find(it)!!.groupValues

            Triple(Direction.findByInitial(direction), steps.toInt(), color)
        }

        println("[Lava Area - Default][$file] ${lines.findDefaultLagoonArea()}")
        println("[Lava Area - Color  ][$file] ${lines.findColorLagoonArea()}")
    }
}

private fun List<Triple<Direction, Int, String>>.findDefaultLagoonArea() =
    map { (direction, steps) -> steps to direction }
        .findLagoonSpace()

private fun List<Triple<Direction, Int, String>>.findColorLagoonArea() =
    map { (_, _, color) ->
        val distanceHex = fromHexDigits(color.substring(0, 5))
        val direction = Direction.findByCode(color.last().digitToInt())

        distanceHex to direction
    }.findLagoonSpace()

private fun List<Pair<Int, Direction>>.findLagoonSpace(): Long {
    return EnclosedSpacesFinder(this).countLagoonSpaces()
}

private class EnclosedSpacesFinder(private val instructions: List<Pair<Int, Direction>>) {
    private data class LineReport(val inside: List<IntRange> = emptyList(), val border: List<IntRange> = emptyList())

    private val directionMap = EnumMap<Direction, Coordinate>(Direction::class.java).also {
        it[LEFT] = -1 to 0
        it[RIGHT] = 1 to 0
        it[UP] = 0 to -1
        it[DOWN] = 0 to 1
    }

    private val sequenceStep = 2000000
    private val coordinatesPerChunk = 2.0.pow(19).toInt()
    private val chunkArraySize = coordinatesPerChunk * 2

    fun countLagoonSpaces() = instructions
        .getBordersPerRow()
        .fold(LineReport() to 0L) { (previous, total), rowBorders ->
            val inside = rowBorders
                .windowed(2) { (l, r) -> l.last + 1 until r.first }
                .clearOutside(previous)

            val nonClosingBorders = rowBorders.excludeClosing(previous)
            val newPoints = inside.sumOf { it.size() } + rowBorders.sumOf { it.size() }

            LineReport(inside, nonClosingBorders) to (total + newPoints)
        }.second

    private fun List<IntRange>.clearOutside(previous: LineReport) =
        filter { xs -> previous.border.any { it.intersects(xs) } || previous.inside.any { it.intersects(xs) } }

    private fun List<IntRange>.excludeClosing(previous: LineReport) =
        filter { border -> previous.inside.none { it.intersects(border) } }

    private fun List<Pair<Int, Direction>>.getBordersPerRow(): Sequence<List<IntRange>> {
        logTimed("Loading Map of Coordinates")
        val coordinateChunks = getMapBorderCoordinates()
        val (min, max) = coordinateChunks.getMinMax()

        return generateSequence(min) { it + sequenceStep }
            .takeWhile { it <= max }
            .flatMap { start ->
                val range = start until start + sequenceStep
                logTimed("Starting [${range.first}, ${range.last}]")

                val steps = Array<MutableList<Int>>(sequenceStep) { mutableListOf() }
                coordinateChunks.forEach { chunk ->
                    (0 until  coordinatesPerChunk)
                        .filter { chunk[(it * 2) + 1] in range }
                        .forEach { steps[chunk[(it * 2) + 1] - start].add(chunk[it * 2]) }
                }

                steps
                    .asSequence()
                    .takeWhile { it.isNotEmpty() }
                    .map { it.findSequentialRanges() }
            }
    }

    private fun List<IntArray>.getMinMax(): Pair<Int, Int> {
        var min = Int.MAX_VALUE
        var max = Int.MIN_VALUE

        forEach {
            min = minOf(min, it[1])
            max = maxOf(max, it[1])
        }
        return min to max
    }

    private fun List<Pair<Int, Direction>>.getMapBorderCoordinates() = asSequence()
        .flatMap { (steps, direction) ->
            val diff = directionMap[direction]!!
            (0 until steps).map { diff }
        }.runningFold(intArrayOf(0, 0)) { (x, y), (xDiff, yDiff) -> intArrayOf(x + xDiff, y + yDiff) }
        .chunked(coordinatesPerChunk) { chunked ->
            IntArray(chunkArraySize) { chunked.getOrNull(it / 2)?.get(it % 2) ?: 0 }
        }.toList()

    private fun List<Int>.findSequentialRanges(): List<IntRange> {
        val sorted = sorted().distinct()
        val sequenceStartingIndices = sorted.windowed(2).mapIndexed { index, (previous, current) ->
            if (previous != current - 1) index + 1
            else null
        }.filterNotNull()

        return (listOf(0) + sequenceStartingIndices + sorted.size)
            .windowed(2) { (start, end) -> sorted[start] .. sorted[end - 1] }
    }
}
