package br.com.gabryel.adventofcode.y2023.d8

fun getNextDestination(
    directions: String,
    mappings: Map<String, Pair<String, String>>,
    current: String,
    steps: Int
) = getNextDestinationByPosition(directions, mappings, current, steps % directions.length)

fun getNextDestinationByPosition(
    directions: String,
    mappings: Map<String, Pair<String, String>>,
    current: String,
    position: Int
): String {
    val nextChoice = mappings[current]
        ?: throw IllegalStateException("No step to take")
    val nextDirection = directions[position]

    return if (nextDirection == 'L') nextChoice.first
    else nextChoice.second
}
