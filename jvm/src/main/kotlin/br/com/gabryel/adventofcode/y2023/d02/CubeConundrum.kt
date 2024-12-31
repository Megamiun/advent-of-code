package br.com.gabryel.adventofcode.y2023.d02

import br.com.gabryel.adventofcode.y2023.d02.Color.*

private enum class Color(val value: String) { R("red"), G("green"), B("blue") }

private val emptyColorMap = mapOf(
    R to 0,
    G to 0,
    B to 0
)

fun getSumOfValidCubeConumdrumGames(lines: List<String>, limits: List<Int>): Int {
    val (redLimit, greenLimit, blueLimit) = limits
    val limitsByColor = mapOf(
        R to redLimit,
        G to greenLimit,
        B to blueLimit,
    )

    return lines.map(::getGameSummary)
        .filter { (_, rounds) -> rounds.isWithinLimit(limitsByColor) }
        .sumOf { it.first }
}

fun getMinimumSetOfCubesForCubeConundrum(lines: List<String>) =
    lines.map(::getGameSummary)
        .map { (_, rounds) -> rounds.getMinimumForColors() }
        .sumOf { it.values.reduce(Int::times) }

private fun List<Map<Color, Int>>.getMinimumForColors() = fold(emptyColorMap) { acc, curr ->
    acc.mapValues { (color, previous) -> maxOf(previous, curr[color] ?: 0) }
}

private fun List<Map<Color, Int>>.isWithinLimit(limits: Map<Color, Int>) = all { round ->
    round.all { (color, number) -> number <= limits.getOrDefault(color, Int.MAX_VALUE) }
}

private fun getGameSummary(line: String): Pair<Int, List<Map<Color, Int>>> {
    val (game, rounds) = line.split(":")
    val (_, gameNum) = game.split(" ")

    val roundSummary = rounds.split(";").map { round ->
        round.split(",").associate { colorData ->
            val (number, colorName) = colorData.trim().split(" ")
            Color.entries.first { colorName == it.value } to number.toInt()
        }
    }

    return gameNum.toInt() to roundSummary
}
