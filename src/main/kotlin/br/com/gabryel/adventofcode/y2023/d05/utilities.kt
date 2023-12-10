package br.com.gabryel.adventofcode.y2023.d05

import br.com.gabryel.adventofcode.y2022.nextIfMatches

fun toDisplacementRange(line: String): Pair<LongRange, Long> {
    val (destinationStart, sourceStart, range) = line.split(" ").map { it.toLong() }
    return (sourceStart until sourceStart + range) to (destinationStart - sourceStart)
}

fun ListIterator<String>.takeUntilNextBlankLine(hasHeader: Boolean = true): List<String> {
    val result = generateSequence { nextIfMatches { it.isNotBlank() } }.toList()
    if (hasNext()) next() // Just to move the cursor

    return if (hasHeader) result.drop(1)
    else result
}
