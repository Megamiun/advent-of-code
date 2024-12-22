package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.getLinesFromSystemIn

fun main() {
    val elfCalories = parseCalories()
    println("Most caloric: ${elfCalories.max()}")
    println("Sum of three most caloric: ${elfCalories.sortedDescending().take(3).sum()}")
}

private fun parseCalories() = generateSequence {
    val singleElfCalories = getAllLines().sum()

    if (singleElfCalories == 0) null
    else singleElfCalories
}.toList()

private fun parseCalories2() = generateSequence {
    getAllLines().ifEmpty { null }?.sum()
}.toList()

private fun getAllLines() = getLinesFromSystemIn(String::toIntOrNull).toList()
