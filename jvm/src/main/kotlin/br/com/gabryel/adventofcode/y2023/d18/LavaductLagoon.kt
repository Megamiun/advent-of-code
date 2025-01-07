package br.com.gabryel.adventofcode.y2023.d18

import br.com.gabryel.adventofcode.util.*
import java.util.HexFormat.fromHexDigits

private typealias Line = Pair<Int, Direction>

private data class LineReport(
    val open: List<IntRange> = emptyList(),
    private val removed: Int = 0,
    private val openTotal: Int = open.sumOf { it.size() }
) {
    val lineTotal = openTotal + removed

    fun replicateOpen() = copy(removed = 0)
}

private val extractor = """(.) (\d+) \(#(.*)\)""".toRegex()

fun findDefaultLagoonAreaFrom(lines: List<String>) = lines.parseLines()
    .map { (direction, steps) -> steps to direction }
    .getFilled()

fun findColorLagoonAreaFrom(lines: List<String>) = lines.parseLines().map { (_, _, color) ->
    val distanceHex = fromHexDigits(color.substring(0, 5))
    val direction = Direction.entries[color.last().digitToInt()]

    distanceHex to direction
}.getFilled()

private fun List<Line>.getFilled(): Long {
    val bordersByRow = getMapBorderCoordinates()
        .groupBy({ it.y() }) { it.x() }

    val min = bordersByRow.keys.min()
    val max = bordersByRow.keys.max()

    return (min..max).asSequence().scan(LineReport()) { report, row ->
        val points = bordersByRow[row] ?: return@scan report.replicateOpen()

        val borders = points.sorted().chunked(2).map { it[0]..it[1] }
        val (removals, additions) = borders
            .partition { border -> report.open.any { open -> border isInside open } }

        val everything = (report.open + additions).mergeIntersections()
        val open = removals.fold(everything) { acc, removed ->
            acc.flatMap { range -> range.removeInclusive(removed).filter { it.size() > 1 } }
        }

        LineReport(open, (removals.sumOf { it.size() } - removals.size))
    }.sumOf { it.lineTotal.toLong() } + 1
}

private fun List<Line>.getMapBorderCoordinates() =
    runningFold(0 to 0) { acc, (step, direction) ->
        acc + (direction.vector * step)
    }.drop(1)

private fun List<IntRange>.mergeIntersections() = sequence {
    val iterator = sortedBy { it.first }.peekingIterator()
    while (iterator.hasNext()) {
        var current = iterator.next()
        while (iterator.hasNext() && iterator.peek() intersects current) {
            current = current.merge(iterator.next())
        }
        yield(current)
    }
}.toList()

private fun List<String>.parseLines() = map {
    val (_, direction, steps, color) = extractor.find(it)!!.groupValues

    Triple(findByDirectionInitial(direction), steps.toInt(), color)
}

private fun findByDirectionInitial(initial: String) = Direction.entries.first { it.name.startsWith(initial) }
