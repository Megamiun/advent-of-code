package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.getLinesFromSystemIn

fun main() {
    val rucksacks = getAllRucksacks()

    val groupPrioritySum = rucksacks.chunked(3).sumOf { group ->
        group
            .map { it.content.toSet() }
            .reduce { acc, content -> acc.intersect(content) }.first()
            .definePriority()
    }

    println("Sum of Rucksacks Priorities: ${rucksacks.sumOf { it.intersectionPriority }}")
    println("Sum of Rucksacks Priorities in Groups of Three: $groupPrioritySum")
}

private fun getAllRucksacks() = getLinesFromSystemIn(::Rucksack).toList()

private class Rucksack(val content: String) {
    private val halfPoint = content.length / 2
    private val left = content.substring(0 until halfPoint)
    private val right = content.substring(halfPoint)

    private val leftSet = left.toSet()
    private val rightSet = right.toSet()

    val intersection = leftSet.intersect(rightSet)

    val intersectionPriority = intersection.sumOf(Char::definePriority)
}

private fun Char.definePriority() =
    if (this >= 'a') 1 + (this - 'a')
    else 27 + (this - 'A')
