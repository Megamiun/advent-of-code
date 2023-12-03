package br.com.gabryel.adventofcode.y2023.d3

import br.com.gabryel.adventofcode.y2023.readLines

fun main() {
    listOf("sample1", "input").forEach { file ->
        val schema = readLines(3, file)

        println("[Adjacent To Symbols][$file] ${schema.getSumOfAdjacents()}")
        println("[Gear Ratios        ][$file] ${schema.getSumOfGearRatios()}")
    }
}

private val numbersFinder = """\d+""".toRegex()
private val gearFinder = """\*""".toRegex()

private fun List<String>.getSumOfGearRatios(): Int {
    val numberAdjacents = getNumbers()
        .map { (line, range, number) -> range.flatMap { col -> getAdjacentCoordinates(line, col) } to number }

    return mapIndexed { index, line -> gearFinder.findAll(line).map { index to it.range.first }.toList() }
        .flatten()
        .map { gear ->
            numberAdjacents
                .filter { (adjacents) -> adjacents.any { gear == it } }
                .map { it.second }
        }.filter { it.size == 2 }
        .sumOf { (first, second) -> first * second }
}

private fun List<String>.getSumOfAdjacents(): Int {
    return getNumbers()
        .filter { (line, range) -> range.any { col -> hasAdjacentSymbol(line, col) }}
        .sumOf { (_, _, value) -> value }
}

private fun List<String>.hasAdjacentSymbol(lineNum: Int, colNum: Int) =
    getAdjacents(lineNum, colNum).any { it != '.' && !it.isDigit() }

private fun List<String>.getAdjacents(
    lineNum: Int,
    colNum: Int
) = getAdjacentCoordinates(lineNum, colNum)
    .mapNotNull { (x, y) -> getOrNull(x)?.getOrNull(y) }
    .distinct()

private fun getAdjacentCoordinates(
    lineNum: Int,
    colNum: Int
) = listOfNotNull(
    lineNum - 1,
    lineNum,
    lineNum + 1
).flatMap { line ->
    listOfNotNull(
        colNum - 1,
        colNum,
        colNum + 1
    ).map { line to it }
}.distinct()

private fun List<String>.getNumbers(): List<Triple<Int, IntRange, Int>> {
    return mapIndexed { lineNum, currLine ->
        numbersFinder.findAll(currLine)
            .map { Triple(lineNum, it.range, it.value.toInt()) }
            .toList()
    }.flatten()
}