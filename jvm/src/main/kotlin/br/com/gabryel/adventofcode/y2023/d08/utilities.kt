package br.com.gabryel.adventofcode.y2023.d08

fun getNextDestination(
    directions: String,
    MAPPINGS: Map<String, Pair<String, String>>,
    current: String,
    steps: Int
) = getNextDestinationByPosition(directions, MAPPINGS, current, steps % directions.length)

fun getNextDestinationByPosition(
    directions: String,
    MAPPINGS: Map<String, Pair<String, String>>,
    current: String,
    position: Int
): String {
    val nextChoice = MAPPINGS[current]
        ?: throw IllegalStateException("No step to take")
    val nextDirection = directions[position]

    return if (nextDirection == 'L') nextChoice.first
    else nextChoice.second
}
