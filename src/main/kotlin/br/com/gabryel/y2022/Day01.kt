package br.com.gabryel.y2022

import java.util.Scanner

private val scanner = Scanner(System.`in`)

fun main() {
    val elfCalories = parseCalories()
    println("Most caloric: ${elfCalories.max()}")
    println("Sum of three most caloric: ${elfCalories.sortedDescending().take(3).sum()}")
}

private fun parseCalories() = generateSequence {
    val singleElfCalories = generateSequence(::getNextMatch).sum()

    if (singleElfCalories == 0) null
    else singleElfCalories
}.toList()

private fun parseCalories2() = generateSequence {
    generateSequence(::getNextMatch)
        .toList()
        .ifEmpty { null }
        ?.sum()
}.toList()

private fun getNextMatch() = if (scanner.hasNextLine()) scanner.nextLine().toIntOrNull() else null

