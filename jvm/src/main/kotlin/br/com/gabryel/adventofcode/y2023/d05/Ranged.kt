package br.com.gabryel.adventofcode.y2023.d05

import br.com.gabryel.adventofcode.util.takeUntilNextBlankLine
import kotlin.math.max
import kotlin.math.min

private typealias Displacer = Pair<LongRange, Long>
private typealias Layer = List<Displacer>
private typealias PointWithDisplacement = Pair<Long, Long>

fun generateSourceToDestinationRanged(lines: List<String>): Long {
    val iterator = lines.listIterator()

    val neededSeeds = iterator.takeUntilNextBlankLine(hasHeader = false)
        .first()
        .removePrefix("seeds: ")
        .split(" ")
        .chunked(2) { (start, range) -> start.toLong() until  start.toLong() + range.toLong() }

    val getFinalLocation = iterator.generatePipeline()
    return neededSeeds.minOf(getFinalLocation)
}

private fun ListIterator<String>.generatePipeline(): (LongRange) -> Long {
    val pipeline = listOf(
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine()
    ).map { it.map(::toDisplacementRange).sortedBy { (range) -> range.first } }

    return { seedRange -> findMinimum(pipeline, seedRange) }
}

private fun findMinimum(remainingLevels: List<Layer>, baseRange: LongRange): Long {
    val level = remainingLevels.firstOrNull()
        ?: return baseRange.first

    val tail = remainingLevels.drop(1)

    val rangeWrappingLevel = level
        .firstOrNull { (levelRange) -> baseRange.last in levelRange && baseRange.first in levelRange }

    if (rangeWrappingLevel != null) {
        val (_, displacement) = rangeWrappingLevel
        return findMinimum(tail, baseRange.displace(displacement))
    }

    val ranges = findPointsOfDisplacement(level, baseRange)
        .windowed(2, partialWindows = false) { (left, right) -> (left.first until right.first).displace(left.second) }

    return ranges.minOf { range -> findMinimum(tail, range) }
}

private fun findPointsOfDisplacement(
    displacers: Layer,
    baseRange: LongRange
): List<PointWithDisplacement> {
    val rangeStarts = displacers.map { (range) -> range.first }

    val distinctPointsOfDisplacement = displacers
        .filter { (range) -> range.last in baseRange || range.first in baseRange }
        .flatMap { (range, displacement) ->
            val start = max(baseRange.first, range.first)
            val end = min(baseRange.last, range.last) + 1

            val startDisplacement = listOf(start to displacement)

            if (end in rangeStarts) startDisplacement
            else startDisplacement + (end to 0L)
        }.toMutableList()

    val firstPoint = distinctPointsOfDisplacement.minOfOrNull { it.first }
    if (firstPoint != baseRange.first)
        distinctPointsOfDisplacement.add(baseRange.first to 0)

    val lastPoint = distinctPointsOfDisplacement.maxOfOrNull { it.first }
    if (lastPoint != baseRange.last)
        distinctPointsOfDisplacement.add(baseRange.last + 1 to 0)

    return distinctPointsOfDisplacement
}

private fun LongRange.displace(displacement: Long) = (first + displacement) .. (last + displacement)
