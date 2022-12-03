package br.com.gabryel.y2022

import java.util.Scanner

private val scanner = Scanner(System.`in`)

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

private fun getAllRucksacks() =
    generateSequence { if (scanner.hasNextLine()) scanner.nextLine().ifEmpty { null } else null }
        .map(::Rucksack)
        .toList()

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
