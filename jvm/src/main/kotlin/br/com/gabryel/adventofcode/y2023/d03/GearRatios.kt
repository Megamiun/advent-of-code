package br.com.gabryel.adventofcode.y2023.d03

private val numbersFinder = """\d+""".toRegex()
private val gearFinder = """\*""".toRegex()

fun getSumOfGearRatios(lines: List<String>): Int {
    val numberAdjacentCoordinates = lines.getNumbers()
        .map { (line, range, number) -> range.flatMap { col -> getAdjacentCoordinates(line, col) } to number }

    val gearsCoordinates = lines.mapIndexed { index, line ->
        gearFinder.findAll(line).map { gear -> index to gear.range.first }.toList()
    }.flatten()

    return gearsCoordinates.map { gearCoordinate ->
        numberAdjacentCoordinates
            .filter { (adjacentCoordinates) -> adjacentCoordinates.contains(gearCoordinate) }
            .map { it.second }
    }.filter { it.size == 2 }
        .sumOf { (first, second) -> first * second }
}

fun getSumOfGearPartNumbers(lines: List<String>) =
    lines.getNumbers()
        .filter { (line, range) -> lines.hasSymbolAround(line, range) }
        .sumOf { (_, _, value) -> value }

private fun List<String>.hasSymbolAround(line: Int, range: IntRange) =
    range.any { col -> hasAdjacentSymbol(line, col) }

private fun List<String>.hasAdjacentSymbol(lineNum: Int, colNum: Int) =
    getValidAdjacentCoordinates(lineNum, colNum).any { it != '.' && !it.isDigit() }

private fun List<String>.getValidAdjacentCoordinates(lineNum: Int, colNum: Int) =
    getAdjacentCoordinates(lineNum, colNum)
        .mapNotNull { (x, y) -> getOrNull(x)?.getOrNull(y) }
        .distinct()

private fun getAdjacentCoordinates(lineNum: Int, colNum: Int) =
    listOf(lineNum - 1, lineNum, lineNum + 1).flatMap { line ->
        listOf(colNum - 1, colNum, colNum + 1).map { line to it }
    }.distinct()

private fun List<String>.getNumbers() =
    flatMapIndexed { lineNum, currLine ->
        numbersFinder.findAll(currLine)
            .map { Triple(lineNum, it.range, it.value.toInt()) }
    }
