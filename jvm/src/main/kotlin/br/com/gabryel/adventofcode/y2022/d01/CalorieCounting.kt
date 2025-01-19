package br.com.gabryel.adventofcode.y2022.d01

fun findCaloriesForTop(elfCalories: List<List<String>>, topElves: Int) = elfCalories
    .map { it.sumOf(String::toInt) }
    .sortedDescending()
    .take(topElves).sum()
