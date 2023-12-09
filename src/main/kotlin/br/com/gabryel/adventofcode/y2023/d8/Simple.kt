package br.com.gabryel.adventofcode.y2023.d8

tailrec fun findStepsToArrive(
    directions: String,
    mappings: Map<String, Pair<String, String>>,
    current: String = "AAA",
    steps: Int = 0
): Int {
    if (current == "ZZZ") return steps

    val nextDestination = getNextDestination(directions, mappings, current, steps)
    return findStepsToArrive(directions, mappings, nextDestination, steps + 1)
}
