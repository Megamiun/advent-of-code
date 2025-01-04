package br.com.gabryel.adventofcode.y2023.d08

import br.com.gabryel.adventofcode.util.leastCommonMultiplier

private val directionRegex = """[0-9A-Z]+""".toRegex()

fun findStepsToArrive(lines: List<String>) =
    findStepsToArrive(lines[0], lines.generateChoices()) { current -> current == "ZZZ" }

fun findGhostStepsToArrive(lines: List<String>): Long {
    val mappings = lines.generateChoices()

    return mappings.keys
        .filter { it.last() == 'A' }
        .map { findStepsToArrive(lines[0], mappings, it) { current -> current.last() == 'Z' }.toLong() }
        .leastCommonMultiplier()
}
private tailrec fun findStepsToArrive(
    directions: String,
    choice: Map<String, Pair<String, String>>,
    current: String = "AAA",
    steps: Int = 0,
    isAtEnd: (String) -> Boolean
): Int {
    if (isAtEnd(current)) return steps

    val nextDestination = getNextDestinationByPosition(directions, choice, current, steps)
    return findStepsToArrive(directions, choice, nextDestination, steps + 1, isAtEnd)
}

fun getNextDestinationByPosition(
    directions: String,
    mappings: Map<String, Pair<String, String>>,
    current: String,
    position: Int
): String {
    val nextChoice = mappings[current]
        ?: throw IllegalStateException("No step to take")
    val nextDirection = directions[position % directions.length]

    return if (nextDirection == 'L') nextChoice.first
    else nextChoice.second
}

private fun List<String>.generateChoices() = drop(1).associate {
    val (start, left, right) = directionRegex.findAll(it).map(MatchResult::value).toList()
    start to (left to right)
}
