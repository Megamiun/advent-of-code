package br.com.gabryel.adventofcode.y2023.d08

tailrec fun findStepsToArrive(
    directions: String,
    MAPPINGS: Map<String, Pair<String, String>>,
    current: String = "AAA",
    steps: Int = 0
): Int {
    if (current == "ZZZ") return steps

    val nextDestination = getNextDestination(directions, MAPPINGS, current, steps)
    return findStepsToArrive(directions, MAPPINGS, nextDestination, steps + 1)
}
