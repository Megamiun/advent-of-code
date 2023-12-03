package br.com.gabryel.adventofcode.y2023.d3

import br.com.gabryel.adventofcode.y2023.readLines

fun main() {
    listOf("sample1", "input").forEach { file ->
        val schema = readLines(3, file)

        println("[Adjacent To Symbols][$file] ${schema.getSumOfAdjacentToSymbols()}")
        println("[Gear Ratios        ][$file] ${schema.getSumOfGearRatios()}")
    }
}

private val numbersFinder = """\d+""".toRegex()
private val gearFinder = """\*""".toRegex()

private fun List<String>.getSumOfGearRatios(): Int {
    val numberAdjacentCoordinates = getNumbers()
        .map { (line, range, number) -> range.flatMap { col -> getAdjacentCoordinates(line, col) } to number }

    val gearsCoordinates = mapIndexed { index, line ->
        gearFinder.findAll(line).map { gear -> index to gear.range.first }.toList()
    }.flatten()

    return gearsCoordinates
        .map { gearCoordinate ->
            numberAdjacentCoordinates
                .filter { (adjacentCoordinates) -> adjacentCoordinates.contains(gearCoordinate) }
                .map { it.second }
        }.filter { it.size == 2 }
        .sumOf { (first, second) -> first * second }
}

private fun List<String>.getSumOfAdjacentToSymbols(): Int {
    return getNumbers()
        .filter { (line, range) -> hasSymbolAround(line, range) }
        .sumOf { (_, _, value) -> value }
}

private fun List<String>.hasSymbolAround(line: Int, range: IntRange) =
    range.any { col -> hasAdjacentSymbol(line, col) }

private fun List<String>.hasAdjacentSymbol(lineNum: Int, colNum: Int) =
    getValidAdjacentCoordinates(lineNum, colNum).any { it != '.' && !it.isDigit() }

private fun List<String>.getValidAdjacentCoordinates(lineNum: Int, colNum: Int) =
    getAdjacentCoordinates(lineNum, colNum)
        .mapNotNull { (x, y) -> getOrNull(x)?.getOrNull(y) }
        .distinct()

private fun getAdjacentCoordinates(lineNum: Int, colNum: Int) =
    listOfNotNull(lineNum - 1, lineNum, lineNum + 1)
        .flatMap { line ->
            listOfNotNull(colNum - 1, colNum, colNum + 1)
                .map { line to it }
        }.distinct()

private fun List<String>.getNumbers(): List<Triple<Int, IntRange, Int>> {
    return mapIndexed { lineNum, currLine ->
        numbersFinder.findAll(currLine)
            .map { Triple(lineNum, it.range, it.value.toInt()) }
            .toList()
    }.flatten()
}