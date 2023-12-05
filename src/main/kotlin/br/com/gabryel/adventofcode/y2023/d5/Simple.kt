package br.com.gabryel.adventofcode.y2023.d5

fun generateSourceToDestination(lines: List<String>): Long {
    val iterator = lines.listIterator()

    val neededSeeds = iterator.takeUntilNextBlankLine(hasHeader = false)
        .first()
        .removePrefix("seeds: ")
        .split(" ")
        .map { it.toLong() }

    val getFinalLocation = iterator.generatePipeline()
    return neededSeeds.minOf(getFinalLocation)
}

private fun ListIterator<String>.generatePipeline(): (Long) -> Long {
    val pipeline = listOf(
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine(),
        takeUntilNextBlankLine()
    ).map { it.map(::toDisplacementRange) }

    return { seed ->
        pipeline.fold(seed) { currentPoint, rangeWithDisplacement ->
            rangeWithDisplacement.firstOrNull { (range) -> currentPoint in range }
                ?.let { (_, displacement) -> currentPoint + displacement }
                ?: currentPoint
        }
    }
}
