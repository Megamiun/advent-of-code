package br.com.gabryel.adventofcode.y2023.d18

import br.com.gabryel.adventofcode.util.Direction
import br.com.gabryel.adventofcode.util.intersects
import br.com.gabryel.adventofcode.util.size
import br.com.gabryel.adventofcode.util.times
import java.util.HexFormat.fromHexDigits
import kotlin.math.pow

private val extractor = """(.) (\d+) \(#(.*)\)""".toRegex()

fun findDefaultLagoonAreaFrom(lines: List<String>) = lines.parseLines().findDefaultLagoonArea()

fun findColorLagoonAreaFrom(lines: List<String>) = lines.parseLines().findColorLagoonArea()

private fun List<Triple<Direction, Int, String>>.findDefaultLagoonArea() =
    map { (direction, steps) -> steps to direction }.findLagoonSpace()

private fun List<Triple<Direction, Int, String>>.findColorLagoonArea() =
    map { (_, _, color) ->
        val distanceHex = fromHexDigits(color.substring(0, 5))
        val direction = Direction.entries[color.last().digitToInt()]

        distanceHex to direction
    }.findLagoonSpace()

private fun List<Pair<Int, Direction>>.findLagoonSpace() =
    EnclosedSpacesFinder(this).countLagoonSpaces()

private class EnclosedSpacesFinder(private val instructions: List<Pair<Int, Direction>>) {
    private val itemsPerChuck = 2.0.pow(19).toInt()
    private val chunkArraySize = itemsPerChuck * 3
    private val rows = 2000000

    private data class LineReport(
        val inside: List<IntRange> = emptyList(),
        val openingBorders: List<IntRange> = emptyList(),
        val closingBorders: List<IntRange> = emptyList()
    )

    fun countLagoonSpaces() = instructions.getBordersPerRow()

    private fun List<Pair<Int, Direction>>.getBordersPerRow(): Long {
        val allBorders = getMapBorderCoordinates()

        val (min, max) = allBorders.getMinMax()

        return generateSequence(min) { it + rows }
            .takeWhile { it <= max }
            .flatMap { min ->
                val range = min..<min + rows

                allBorders.flatMapIndexed { allBordersIndex, borders ->
                    (0 until itemsPerChuck)
                        .filter { borders[it * 3] in range }
                        .map { allBordersIndex to it * 3 }
                }.groupBy({ (allBordersIndex, internalIndex) -> allBorders[allBordersIndex][internalIndex] }) { (allBordersIndex, internalIndex) -> allBorders[allBordersIndex][internalIndex + 1]..allBorders[allBordersIndex][internalIndex + 2] }
                    .toSortedMap()
                    .values
            }.fold(LineReport() to 0L) { (report, total), row ->
                val borders = row.distinct().sortedBy { it.first }

                val insideAndOpening = report.inside + report.openingBorders

                val inside = borders.windowed(2) { (f, s) -> (f.last + 1)..<s.first }
                    .filter { it.size() > 0 }
                    .filter { inner -> insideAndOpening.any { range -> range.intersects(inner) } }

                val all = insideAndOpening + report.closingBorders
                val (closing, opening) = borders.partition { border -> all.any { range -> range.intersects(border) } }

                val newReport = LineReport(inside, opening, closing)
                newReport to (total + inside.sumOf { it.size() } + borders.sumOf { it.size() })
            }.second
    }

    private fun List<IntArray>.getMinMax(): Pair<Int, Int> {
        var min = Int.MAX_VALUE
        var max = Int.MIN_VALUE

        forEach { border ->
            for (it in 0 until itemsPerChuck) {
                if (border[it * 3] == Int.MIN_VALUE)
                    break

                min = minOf(min, border[it * 3])
                max = maxOf(max, border[it * 3])
            }
        }
        return min to max
    }

    private fun List<Pair<Int, Direction>>.getMapBorderCoordinates() = asSequence()
        .flatMap { (steps, direction) ->
            if (direction.horizontal)
                listOf(direction.vector * steps)
            else
                (0 until steps).map { direction.vector }
        }.runningFold(Triple(0, 0, (0..0))) { (col, row), (stepX, stepY) ->
            Triple(col + stepX, row + stepY, sortedRange(col, stepX))
        }.chunked(itemsPerChuck) { chuckData ->
            createChunck(chuckData)
        }.toList()

    private fun createChunck(chuckData: List<Triple<Int, Int, IntRange>>): IntArray {
        val chunck = IntArray(chunkArraySize) { Int.MIN_VALUE }
        chuckData.forEachIndexed { index, value ->
            val base = index * 3
            chunck[base] = value.second
            chunck[base + 1] = value.third.first
            chunck[base + 2] = value.third.last
        }
        return chunck
    }

    private fun sortedRange(last: Int, stepX: Int) =
        if (stepX == 0)
            last..last
        else if (stepX > 0)
            last + 1..(last + stepX)
        else
            (last + stepX)..<last
}

private fun List<String>.parseLines() = map {
    val (_, direction, steps, color) = extractor.find(it)!!.groupValues

    Triple(findByDirectionInitial(direction), steps.toInt(), color)
}

private fun findByDirectionInitial(initial: String) = Direction.entries.first { it.name.startsWith(initial) }
