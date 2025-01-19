package br.com.gabryel.adventofcode.y2022.d03

fun getRucksackRepeatedPriorities(lines: List<String>) = lines.parse().sumOf { it.intersectionPriority }

fun getRucksackRepeatedPrioritiesBetweenTrios(lines: List<String>) = lines.chunked(3).sumOf { group ->
    group
        .map { it.toSet() }
        .reduce { acc, content -> acc.intersect(content) }.first()
        .definePriority()
}

private class Rucksack(content: String) {
    private val halfPoint = content.length / 2

    private val leftSet = content.substring(0 until halfPoint).toSet()
    private val rightSet = content.substring(halfPoint).toSet()

    val intersection = leftSet.intersect(rightSet)

    val intersectionPriority = intersection.sumOf(Char::definePriority)
}

private fun Char.definePriority() =
    if (this >= 'a') (this - '`')
    else 26 + (this - '@')

private fun List<String>.parse() = map(::Rucksack).toList()
