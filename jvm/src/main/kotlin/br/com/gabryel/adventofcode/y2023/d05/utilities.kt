package br.com.gabryel.adventofcode.y2023.d05

fun toDisplacementRange(line: String): Pair<LongRange, Long> {
    val (destinationStart, sourceStart, range) = line.split(" ").map { it.toLong() }
    return (sourceStart until sourceStart + range) to (destinationStart - sourceStart)
}

