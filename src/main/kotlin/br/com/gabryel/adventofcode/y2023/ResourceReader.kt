package br.com.gabryel.adventofcode.y2023

object ResourceReader

fun readLines(day: Int, file: String) =
    readFile(day, file)
        .lines()
        .filter { it.isNotBlank() }

fun readFile(day: Int, file: String) =
    ResourceReader.javaClass.getResourceAsStream("d$day/$file")
        ?.bufferedReader()?.readText()
        ?: throw IllegalStateException("""¨d$day/$file¨ not found""")
